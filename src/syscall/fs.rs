use alloc::string::String;
use axerrno::{AxError, AxResult, LinuxError};
use axfs::{FsContext, OpenOptions};
use axfs_ng_vfs::VfsError;
use axio::{Seek, SeekFrom};
use linux_raw_sys::general::{stat, O_APPEND, O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, S_IFREG};

use super::{linux_err_to_isize, fd_table, ENOSYS};

pub const AT_FDCWD: isize = -100;
const MAX_PATH: usize = 4096;

/// Get the global filesystem context
fn get_fs_context() -> AxResult<FsContext> {
    axfs::ROOT_FS_CONTEXT
        .get()
        .cloned()
        .ok_or(AxError::NotFound)
}

/// Load a null-terminated C string from user space
fn load_user_cstring(ptr: usize) -> AxResult<String> {
    if ptr == 0 {
        return Err(AxError::BadAddress);
    }
    
    let mut bytes = alloc::vec::Vec::new();
    for i in 0..MAX_PATH {
        let byte = unsafe { *(ptr.wrapping_add(i) as *const u8) };
        if byte == 0 {
            return String::from_utf8(bytes)
                .map_err(|_| AxError::InvalidData);
        }
        bytes.push(byte);
    }
    
    Err(AxError::InvalidInput) // Path too long
}

/// Convert Linux open flags to OpenOptions
fn make_open_options(flags: u32) -> OpenOptions {
    let mut opts = OpenOptions::new();
    
    // Handle access mode (bits 0-1)
    match flags & 0o3 {
        O_RDONLY => opts.read(true),
        O_WRONLY => opts.write(true),
        _ => opts.read(true).write(true),
    };
    
    // Handle flags
    if flags & O_APPEND != 0 {
        opts.append(true);
    }
    if flags & O_TRUNC != 0 {
        opts.truncate(true);
    }
    if flags & O_CREAT != 0 {
        opts.create(true);
    }
    
    opts
}

/// Syscall: openat(2) - Open or create a file
/// Returns file descriptor on success, negative error on failure
pub fn sys_openat(dirfd: isize, path: usize, flags: usize, _mode: usize) -> isize {
    if dirfd != AT_FDCWD {
        return linux_err_to_isize(LinuxError::EINVAL);
    }
    
    let path_str = match load_user_cstring(path) {
        Ok(s) => s,
        Err(e) => return ax_err_to_isize(e),
    };
    
    let fs = match get_fs_context() {
        Ok(fs) => fs,
        Err(e) => return ax_err_to_isize(e),
    };
    
    let opts = make_open_options(flags as u32);
    let file = match opts.open(&fs, &path_str) {
        Ok(result) => match result.into_file() {
            Ok(f) => f,
            Err(e) => return vfs_err_to_isize(e),
        },
        Err(e) => return vfs_err_to_isize(e),
    };
    
    fd_table::add_file(file) as isize
}

/// Syscall: read(2) - Read from a file
/// Returns number of bytes read on success, negative error on failure
pub fn sys_read(fd: i32, buf: usize, count: usize) -> isize {
    let res = fd_table::with_file_mut(fd, |file| {
        let slice = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };
        file.read(slice)
    });
    
    match res {
        Some(Ok(n)) => n as isize,
        Some(Err(_e)) => linux_err_to_isize(LinuxError::EIO),
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

/// Syscall: write(2) - Write to a file
/// Returns number of bytes written on success, negative error on failure
pub fn sys_write(fd: i32, buf: usize, count: usize) -> isize {
    let res = fd_table::with_file_mut(fd, |file| {
        let slice = unsafe { core::slice::from_raw_parts(buf as *const u8, count) };
        file.write(slice)
    });
    
    match res {
        Some(Ok(n)) => n as isize,
        Some(Err(_e)) => linux_err_to_isize(LinuxError::EIO),
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

/// Syscall: close(2) - Close a file descriptor
/// Returns 0 on success, negative error on failure
pub fn sys_close(fd: i32) -> isize {
    if fd_table::remove_file(fd).is_some() {
        0
    } else {
        linux_err_to_isize(LinuxError::EBADF)
    }
}

/// Syscall: lseek(2) - Change file position
/// Returns new offset on success, negative error on failure
pub fn sys_lseek(fd: i32, offset: isize, whence: i32) -> isize {
    let res = fd_table::with_file_mut(fd, |file| {
        let seek_from = match whence {
            0 => SeekFrom::Start(offset as u64),
            1 => SeekFrom::Current(offset as i64),
            2 => SeekFrom::End(offset as i64),
            _ => return Err(AxError::InvalidInput),
        };
        
        (&*file).seek(seek_from)
            .map(|pos| pos as isize)
            .map_err(|_| AxError::InvalidInput)
    });
    
    match res {
        Some(Ok(pos)) => pos,
        Some(Err(_)) => linux_err_to_isize(LinuxError::EINVAL),
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

/// Syscall: fstat(2) - Get file status
/// Returns 0 on success, negative error on failure
pub fn sys_fstat(fd: i32, statbuf: usize) -> isize {
    let res = fd_table::with_file_mut(fd, |file| -> Result<i32, AxError> {
        // Get current position
        let cur_pos = (&*file).seek(SeekFrom::Current(0))
            .map_err(|_| AxError::InvalidInput)?;
        
        // Get file size
        let end_pos = (&*file).seek(SeekFrom::End(0))
            .map_err(|_| AxError::InvalidInput)?;
        
        // Restore position
        (&*file).seek(SeekFrom::Start(cur_pos))
            .map_err(|_| AxError::InvalidInput)?;
        
        // Build stat struct
        let mut st: stat = unsafe { core::mem::zeroed() };
        st.st_mode = (S_IFREG | 0o666) as _;
        st.st_nlink = 1;
        st.st_size = end_pos as _;
        st.st_blksize = 4096;
        
        // Write to user space
        unsafe { (statbuf as *mut stat).write(st); }
        
        Ok(0)
    });
    
    match res {
        Some(Ok(_)) => 0,
        Some(Err(_)) => linux_err_to_isize(LinuxError::EIO),
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

/// Syscall: fcntl(2) - File control operations
/// Supported commands:
/// - F_GETFD: Get file descriptor flags
/// - F_SETFD: Set file descriptor flags
/// - F_GETFL: Get file status flags
/// - F_SETFL: Set file status flags  
/// - F_DUPFD: Duplicate file descriptor
pub fn sys_fcntl(fd: i32, cmd: i32, arg: usize) -> isize {
    const F_GETFD: i32 = 1;
    const F_SETFD: i32 = 2;
    const F_GETFL: i32 = 3;
    const F_SETFL: i32 = 4;
    const F_DUPFD: i32 = 0;
    
    // Verify fd is valid first
    if fd_table::get_fd_flags(fd).is_none() && cmd != F_DUPFD {
        return linux_err_to_isize(LinuxError::EBADF);
    }
    
    match cmd {
        F_GETFD => {
            // Get file descriptor flags (FD_CLOEXEC etc.)
            match fd_table::get_fd_flags(fd) {
                Some(flags) => flags as isize,
                None => linux_err_to_isize(LinuxError::EBADF),
            }
        }
        F_SETFD => {
            // Set file descriptor flags
            if fd_table::set_fd_flags(fd, arg as u32) {
                0
            } else {
                linux_err_to_isize(LinuxError::EBADF)
            }
        }
        F_GETFL => {
            // Get file status flags (O_APPEND, O_NONBLOCK etc.)
            match fd_table::get_file_flags(fd) {
                Some(flags) => flags as isize,
                None => linux_err_to_isize(LinuxError::EBADF),
            }
        }
        F_SETFL => {
            // Set file status flags
            if fd_table::set_file_flags(fd, arg as u32) {
                0
            } else {
                linux_err_to_isize(LinuxError::EBADF)
            }
        }
        F_DUPFD => {
            // Duplicate file descriptor (not fully implemented due to File not being Clone-able)
            // Would need Arc<File> or similar in production
            ENOSYS
        }
        _ => ENOSYS,
    }
}

/// Helper: Convert AxError to LinuxError and then to isize
fn ax_err_to_isize(err: AxError) -> isize {
    let linux_err = LinuxError::from(err);
    linux_err_to_isize(linux_err)
}

/// Helper: Convert VfsError to LinuxError and then to isize
fn vfs_err_to_isize(err: VfsError) -> isize {
    let linux_err = match err {
        VfsError::NotFound => LinuxError::ENOENT,
        VfsError::PermissionDenied => LinuxError::EACCES,
        VfsError::IsADirectory => LinuxError::EISDIR,
        VfsError::NotADirectory => LinuxError::ENOTDIR,
        VfsError::AlreadyExists => LinuxError::EEXIST,
        VfsError::InvalidInput => LinuxError::EINVAL,
        VfsError::FilesystemLoop => LinuxError::ELOOP,
        VfsError::ReadOnlyFilesystem => LinuxError::EROFS,
        _ => LinuxError::EIO,
    };
    linux_err_to_isize(linux_err)
}
