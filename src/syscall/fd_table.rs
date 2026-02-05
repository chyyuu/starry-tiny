use alloc::vec::Vec;
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use arceos_api::fs::AxFileHandle;

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> core::ops::Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

pub struct FdTable {
    entries: Vec<Option<AxFileHandle>>,
}

impl FdTable {
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn insert(&mut self, file: AxFileHandle) -> i32 {
        for (idx, entry) in self.entries.iter_mut().enumerate() {
            if entry.is_none() {
                *entry = Some(file);
                return idx as i32;
            }
        }
        self.entries.push(Some(file));
        (self.entries.len() - 1) as i32
    }

    fn get_mut(&mut self, fd: i32) -> Option<&mut AxFileHandle> {
        if fd < 0 {
            return None;
        }
        self.entries.get_mut(fd as usize)?.as_mut()
    }

    fn remove(&mut self, fd: i32) -> Option<AxFileHandle> {
        if fd < 0 {
            return None;
        }
        let entry = self.entries.get_mut(fd as usize)?;
        entry.take()
    }
}

static FD_TABLE: SpinLock<FdTable> = SpinLock::new(FdTable::new());

pub fn add_file(file: AxFileHandle) -> i32 {
    FD_TABLE.lock().insert(file)
}

pub fn with_file_mut<R>(fd: i32, f: impl FnOnce(&mut AxFileHandle) -> R) -> Option<R> {
    let mut table = FD_TABLE.lock();
    let file = table.get_mut(fd)?;
    Some(f(file))
}

pub fn remove_file(fd: i32) -> Option<AxFileHandle> {
    FD_TABLE.lock().remove(fd)
}
