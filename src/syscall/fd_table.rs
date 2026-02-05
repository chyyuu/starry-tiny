/// File descriptor table implementation
/// 
/// Manages open file descriptors using a simple Vec-based table with dynamic growth.

use alloc::vec::Vec;
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use axfs::File;

/// Simple spinlock for fd_table protection
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

/// File descriptor table
pub struct FdTable {
    entries: Vec<Option<File>>,
}

impl FdTable {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Insert a file and return its file descriptor
    fn insert(&mut self, file: File) -> i32 {
        for (idx, entry) in self.entries.iter_mut().enumerate() {
            if entry.is_none() {
                *entry = Some(file);
                return idx as i32;
            }
        }
        self.entries.push(Some(file));
        (self.entries.len() - 1) as i32
    }

    /// Get mutable reference to a file by fd
    fn get_mut(&mut self, fd: i32) -> Option<&mut File> {
        if fd < 0 {
            return None;
        }
        self.entries.get_mut(fd as usize)?.as_mut()
    }

    /// Remove and return a file by fd
    fn remove(&mut self, fd: i32) -> Option<File> {
        if fd < 0 {
            return None;
        }
        let entry = self.entries.get_mut(fd as usize)?;
        entry.take()
    }
}

static FD_TABLE: SpinLock<FdTable> = SpinLock::new(FdTable::new());

/// Add a file to the fd table and return its fd
pub fn add_file(file: File) -> i32 {
    FD_TABLE.lock().insert(file)
}

/// Execute a closure with mutable access to a file
pub fn with_file_mut<R, F>(fd: i32, f: F) -> Option<R>
where
    F: FnOnce(&mut File) -> R,
{
    let mut table = FD_TABLE.lock();
    let file = table.get_mut(fd)?;
    Some(f(file))
}

/// Remove a file from the fd table
pub fn remove_file(fd: i32) -> Option<File> {
    FD_TABLE.lock().remove(fd)
}

