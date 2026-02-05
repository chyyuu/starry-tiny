use axtask::exit as ax_exit;

/// Syscall: exit(2) - Terminate the current task
/// This function never returns
pub fn sys_exit(code: i32) -> isize {
    ax_exit(code as i32);
}

/// Syscall: exit_group(2) - Terminate the entire process group
/// In single-process mode, this is equivalent to exit()
/// This function never returns
pub fn sys_exit_group(code: i32) -> isize {
    ax_exit(code as i32);
}


