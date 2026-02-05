use super::ENOSYS;

pub fn sys_exit(code: i32) -> isize {
    let _ = code;
    ENOSYS
}

pub fn sys_exit_group(code: i32) -> isize {
    let _ = code;
    ENOSYS
}
