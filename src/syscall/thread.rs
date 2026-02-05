// Simple process/thread ID tracking
static mut TID: usize = 1;
static mut TID_ADDRESS: usize = 0;

/// Syscall: set_tid_address(2)
/// Sets the address where the kernel writes the thread ID when the thread exits
/// Returns the thread ID
pub fn sys_set_tid_address(tidp: usize) -> isize {
    // In single-threaded mode, tid == pid == 1
    unsafe {
        TID_ADDRESS = tidp;
        // Write tid to user memory if address is valid
        if tidp != 0 {
            *(tidp as *mut i32) = TID as i32;
        }
        TID as isize
    }
}

/// Get current thread ID (for internal use)
#[allow(dead_code)]
pub fn get_tid() -> usize {
    unsafe { TID }
}

/// Get stored tid address (for internal use)
#[allow(dead_code)]
pub fn get_tid_address() -> usize {
    unsafe { TID_ADDRESS }
}
