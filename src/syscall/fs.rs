use super::ENOSYS;

pub const AT_FDCWD: isize = -100;

pub fn sys_openat(dirfd: isize, path: usize, flags: usize, mode: usize) -> isize {
    let _ = (dirfd, path, flags, mode);
    ENOSYS
}

pub fn sys_read(fd: i32, buf: usize, count: usize) -> isize {
    let _ = (fd, buf, count);
    ENOSYS
}

pub fn sys_write(fd: i32, buf: usize, count: usize) -> isize {
    let _ = (fd, buf, count);
    ENOSYS
}

pub fn sys_close(fd: i32) -> isize {
    let _ = fd;
    ENOSYS
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
