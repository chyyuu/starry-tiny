use super::ENOSYS;

static mut BRK_END: usize = 0;

pub fn sys_brk(addr: usize) -> isize {
    unsafe {
        if addr == 0 {
            if BRK_END == 0 {
                BRK_END = 0x10000;
            }
            BRK_END as isize
        } else {
            if addr > BRK_END {
                BRK_END = addr;
            }
            BRK_END as isize
        }
    }
}

pub fn sys_mmap(addr: usize, len: usize, _prot: usize, _flags: usize, _fd: usize, _offset: usize) -> isize {
    let _ = (addr, len);
    ENOSYS
}
