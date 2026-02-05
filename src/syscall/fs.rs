use alloc::{string::String, vec::Vec};

use arceos_api::fs::{ax_open_file, ax_read_file, ax_write_file, AxOpenOptions};
use axerrno::{AxError, AxResult, LinuxError};
use linux_raw_sys::general::{O_APPEND, O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY};

use super::{ax_err_to_isize, fd_table, linux_err_to_isize, ENOSYS};

pub const AT_FDCWD: isize = -100;
const MAX_PATH: usize = 4096;

fn flags_to_options(flags: usize) -> AxOpenOptions {
    let mut opts = AxOpenOptions::new();
    match flags as u32 & 0b11 {
        O_RDONLY => opts.read(true),
        O_WRONLY => opts.write(true),
        _ => opts.read(true).write(true),
    };
    if flags as u32 & O_APPEND != 0 {
        opts.append(true);
    }
    if flags as u32 & O_TRUNC != 0 {
        opts.truncate(true);
    }
    if flags as u32 & O_CREAT != 0 {
        opts.create(true);
    }
    opts
}

fn load_user_cstring(ptr: usize) -> AxResult<String> {
    if ptr == 0 {
        return Err(AxError::BadAddress);
    }
    let mut bytes = Vec::new();
    for i in 0..MAX_PATH {
        let c = unsafe { *(ptr as *const u8).add(i) };
        if c == 0 {
            break;
        }
        bytes.push(c);
    }
    if bytes.len() == MAX_PATH {
        return Err(AxError::InvalidInput);
    }
    String::from_utf8(bytes).map_err(|_| AxError::InvalidData)
}

pub fn sys_openat(dirfd: isize, path: usize, flags: usize, _mode: usize) -> isize {
    if dirfd != AT_FDCWD {
        return linux_err_to_isize(LinuxError::EINVAL);
    }
    let path = match load_user_cstring(path) {
        Ok(path) => path,
        Err(err) => return ax_err_to_isize(err),
    };
    let opts = flags_to_options(flags);
    match ax_open_file(&path, &opts) {
        Ok(file) => fd_table::add_file(file) as isize,
        Err(err) => ax_err_to_isize(err),
    }
}

pub fn sys_read(fd: i32, buf: usize, count: usize) -> isize {
    let res = fd_table::with_file_mut(fd, |file| unsafe {
        let slice = core::slice::from_raw_parts_mut(buf as *mut u8, count);
        ax_read_file(file, slice)
    });
    match res {
        Some(Ok(n)) => n as isize,
        Some(Err(err)) => ax_err_to_isize(err),
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

pub fn sys_write(fd: i32, buf: usize, count: usize) -> isize {
    let res = fd_table::with_file_mut(fd, |file| unsafe {
        let slice = core::slice::from_raw_parts(buf as *const u8, count);
        ax_write_file(file, slice)
    });
    match res {
        Some(Ok(n)) => n as isize,
        Some(Err(err)) => ax_err_to_isize(err),
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

pub fn sys_close(fd: i32) -> isize {
    match fd_table::remove_file(fd) {
        Some(_) => 0,
        None => linux_err_to_isize(LinuxError::EBADF),
    }
}

pub fn sys_fstat(fd: i32, statbuf: usize) -> isize {
    let _ = (fd, statbuf);
    ENOSYS
}

pub fn sys_lseek(fd: i32, offset: isize, whence: i32) -> isize {
    let _ = (fd, offset, whence);
    ENOSYS
}

pub fn sys_fcntl(fd: i32, cmd: i32, arg: usize) -> isize {
    let _ = (fd, cmd, arg);
    ENOSYS
}
