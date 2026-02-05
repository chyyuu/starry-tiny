use linux_raw_sys::general::{stat, O_APPEND, O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, S_IFREG};
use axerrno::LinuxError;

use super::{fd_table, linux_err_to_isize, ENOSYS};

pub const AT_FDCWD: isize = -100;

/// Syscall: openat(2)
pub fn sys_openat(_dirfd: isize, _path: usize, _flags: usize, _mode: usize) -> isize {
    linux_err_to_isize(LinuxError::ENOENT)
}

/// Syscall: read(2)
pub fn sys_read(_fd: i32, _buf: usize, _count: usize) -> isize {
    ENOSYS
}

/// Syscall: write(2)
pub fn sys_write(_fd: i32, _buf: usize, _count: usize) -> isize {
    ENOSYS
}

/// Syscall: close(2)
pub fn sys_close(_fd: i32) -> isize {
    ENOSYS
}

/// Syscall: lseek(2)
pub fn sys_lseek(_fd: i32, _offset: isize, _whence: i32) -> isize {
    ENOSYS
}

/// Syscall: fstat(2)
pub fn sys_fstat(_fd: i32, _statbuf: usize) -> isize {
    ENOSYS
}

/// Syscall: fcntl(2)
pub fn sys_fcntl(_fd: i32, _cmd: i32, _arg: usize) -> isize {
    ENOSYS
}
