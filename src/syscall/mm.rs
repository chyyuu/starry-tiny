use super::ENOSYS;

pub fn sys_brk(addr: usize) -> isize {
    let _ = addr;
    ENOSYS
}

pub fn sys_mmap(addr: usize, len: usize, prot: usize, flags: usize, fd: usize, offset: usize) -> isize {
    let _ = (addr, len, prot, flags, fd, offset);
    ENOSYS
}
