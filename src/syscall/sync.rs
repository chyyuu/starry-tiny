use super::ENOSYS;

pub fn sys_set_robust_list(head: usize, len: usize) -> isize {
    let _ = (head, len);
    ENOSYS
}
