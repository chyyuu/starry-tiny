use super::ENOSYS;

pub fn sys_set_tid_address(tidp: usize) -> isize {
    let _ = tidp;
    ENOSYS
}
