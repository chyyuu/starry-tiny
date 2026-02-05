use super::ENOSYS;

pub fn sys_exit(_code: i32) -> isize {
    // TODO: Implement exit
    ENOSYS
}

pub fn sys_exit_group(_code: i32) -> isize {
    // TODO: Implement exit_group
    ENOSYS
}
