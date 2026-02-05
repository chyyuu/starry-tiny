

static mut ROBUST_LIST_HEAD: usize = 0;

/// Syscall: set_robust_list(2)
/// Used for robust mutex support (futex-based)
/// In single-threaded mode, we just track the pointer
pub fn sys_set_robust_list(head: usize, len: usize) -> isize {
    // len should be sizeof(struct robust_list_head) = 16
    if len != 16 {
        return -22; // EINVAL
    }
    
    unsafe {
        ROBUST_LIST_HEAD = head;
    }
    0
}

/// Get the robust list head (for internal use)
#[allow(dead_code)]
pub fn get_robust_list_head() -> usize {
    unsafe { ROBUST_LIST_HEAD }
}
