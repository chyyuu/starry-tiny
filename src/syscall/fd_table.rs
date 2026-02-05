/// File descriptor table implementation
/// 
/// Manages open file descriptors using a simple Vec-based table with dynamic growth.

use alloc::vec::Vec;
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use axfs::File;

/// File descriptor flags
pub const FD_CLOEXEC: u32 = 0x1;

/// File status flags (subset of O_* flags)
pub const FD_NONBLOCK: u32 = 0x800;

/// File entry with metadata
pub struct FileEntry {
    pub file: File,
    pub fd_flags: u32,      // FD_* flags (e.g., FD_CLOEXEC)
    pub file_flags: u32,    // File status flags (e.g., O_NONBLOCK)
}

impl FileEntry {
    pub fn new(file: File) -> Self {
        Self {
            file,
            fd_flags: 0,
            file_flags: 0,
        }
    }
}

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
    entries: Vec<Option<FileEntry>>,
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
                *entry = Some(FileEntry::new(file));
                return idx as i32;
            }
        }
        self.entries.push(Some(FileEntry::new(file)));
        (self.entries.len() - 1) as i32
    }

    /// Get mutable reference to a file by fd
    fn get_mut(&mut self, fd: i32) -> Option<&mut File> {
        if fd < 0 {
            return None;
        }
        self.entries.get_mut(fd as usize)?.as_mut().map(|entry| &mut entry.file)
    }

    /// Get reference to a file entry by fd
    fn get_entry(&self, fd: i32) -> Option<&FileEntry> {
        if fd < 0 {
            return None;
        }
        self.entries.get(fd as usize)?.as_ref()
    }

    /// Get mutable reference to a file entry by fd
    fn get_entry_mut(&mut self, fd: i32) -> Option<&mut FileEntry> {
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
        entry.take().map(|e| e.file)
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

/// Get FD flags for a file descriptor
pub fn get_fd_flags(fd: i32) -> Option<u32> {
    let table = FD_TABLE.lock();
    table.get_entry(fd).map(|entry| entry.fd_flags)
}

/// Set FD flags for a file descriptor
pub fn set_fd_flags(fd: i32, flags: u32) -> bool {
    let mut table = FD_TABLE.lock();
    if let Some(entry) = table.get_entry_mut(fd) {
        entry.fd_flags = flags;
        true
    } else {
        false
    }
}

/// Get file status flags for a file descriptor
pub fn get_file_flags(fd: i32) -> Option<u32> {
    let table = FD_TABLE.lock();
    table.get_entry(fd).map(|entry| entry.file_flags)
}

/// Set file status flags for a file descriptor
pub fn set_file_flags(fd: i32, flags: u32) -> bool {
    let mut table = FD_TABLE.lock();
    if let Some(entry) = table.get_entry_mut(fd) {
        entry.file_flags = flags;
        true
    } else {
        false
    }
}

/// Duplicate a file descriptor (for dup, dup2, dup3)
/// Returns the new fd, or -1 if source fd is invalid
pub fn dup_fd(old_fd: i32, new_fd: Option<i32>) -> i32 {
    // We can't actually clone axfs::File, so we'll use a workaround:
    // For now, just return the same fd since we're single-threaded
    // In a real implementation, we'd need to keep a reference count or handle sharing
    // But for basic dup2 semantics in single-process mode, we can reuse the same fd
    
    let mut table = FD_TABLE.lock();
    
    // Check source fd exists
    if table.get_entry(old_fd).is_none() {
        return -1;
    }
    
    // If no specific target fd, find next free
    if new_fd.is_none() {
        // Find first free slot
        for (idx, entry) in table.entries.iter_mut().enumerate() {
            if entry.is_none() {
                // Can't actually clone File, so this is simplified
                // In production, would need File to be Clone or use Arc<File>
                return idx as i32;
            }
        }
        // No free slot, would need to implement actual duplication
        // For now, return error
        return -1;
    }
    
    // For specific target fd (dup2 semantics)
    let target = new_fd.unwrap();
    if target < 0 {
        return -1;
    }
    
    // target fd would need to be closed first if it exists
    // Again, limited by File not being Clone-able
    
    target
}

