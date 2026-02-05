use arceos_api::task::ax_exit;

pub fn sys_exit(code: i32) -> isize {
    ax_exit(code);
}

pub fn sys_exit_group(code: i32) -> isize {
    // Single-process mode: exit_group is equivalent to exit.
    ax_exit(code);
}
