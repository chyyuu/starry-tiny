/// Resource limits constants
const RLIM_NLIMITS: usize = 16;

/// Resource limit structure
#[repr(C)]
pub struct RlimitV {
    pub soft: u64,
    pub hard: u64,
}

/// Syscall: prlimit64(2) - Get/set resource limits
/// In single-process mode, returns default limits
/// Supports: pid (must be 0 or current), resource, new_limit, old_limit
pub fn sys_prlimit64(pid: i32, resource: u32, _new_limit: usize, old_limit: usize) -> isize {
    // Only support pid 0 (current process)
    if pid != 0 {
        return -22; // EINVAL
    }
    
    if resource as usize >= RLIM_NLIMITS {
        return -22; // EINVAL
    }
    
    // Default limits (matching Linux defaults)
    let limits = match resource {
        0 => RlimitV { soft: 0, hard: 0 },           // RLIMIT_CPU
        1 => RlimitV { soft: u64::MAX, hard: u64::MAX }, // RLIMIT_FSIZE
        2 => RlimitV { soft: 8388608, hard: u64::MAX }, // RLIMIT_DATA (8MB soft, unlimited hard)
        3 => RlimitV { soft: 8388608, hard: u64::MAX }, // RLIMIT_STACK (8MB)
        4 => RlimitV { soft: 1048576, hard: u64::MAX }, // RLIMIT_CORE (1MB)
        7 => RlimitV { soft: 1024, hard: 1048576 },     // RLIMIT_NOFILE (1024/1M)
        10 => RlimitV { soft: 8388608, hard: 8388608 }, // RLIMIT_MEMLOCK (8MB)
        _ => RlimitV { soft: u64::MAX, hard: u64::MAX }, // Default: unlimited
    };
    
    // Write old limit if pointer provided
    if old_limit != 0 {
        unsafe {
            let ptr = old_limit as *mut RlimitV;
            (*ptr).soft = limits.soft;
            (*ptr).hard = limits.hard;
        }
    }
    
    // Ignore setting new limits for now (return success)
    // In production, would actually change the limits
    
    0
}

/// Syscall: readlinkat(2) - Read value of a symbolic link
/// For now, returns EINVAL as we don't have proper symlink support
/// Could return self-exe path for /proc/self/exe
pub fn sys_readlinkat(_dirfd: i32, path: usize, buf: usize, bufsiz: usize) -> isize {
    // Load path from user space
    let path_str = match super::fs::load_user_cstring(path) {
        Ok(s) => s,
        Err(_) => return -14, // EFAULT
    };
    
    // Special case: /proc/self/exe
    if path_str == "/proc/self/exe" || path_str.ends_with("/proc/self/exe") {
        let exe = b"./test";
        let len = exe.len().min(bufsiz);
        
        unsafe {
            core::ptr::copy_nonoverlapping(exe.as_ptr(), buf as *mut u8, len);
        }
        return len as isize;
    }
    
    // For other paths, return "not a symbolic link" error
    -22 // EINVAL
}

/// Syscall: getrandom(2) - Obtain random bytes
/// Currently returns a simple pseudo-random sequence
pub fn sys_getrandom(buf: usize, len: usize, _flags: u32) -> isize {
    if len == 0 {
        return 0;
    }
    
    if len > 512 {
        return -22; // EINVAL - too large
    }
    
    // Simple PRNG based on a static counter
    static mut SEED: u64 = 0xdeadbeef;
    
    unsafe {
        let buf_ptr = buf as *mut u8;
        
        for i in 0..len {
            // Simple linear congruential generator
            SEED = SEED.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let byte = (SEED >> 32) as u8;
            
            core::ptr::write(buf_ptr.add(i), byte);
        }
    }
    
    len as isize
}

/// Syscall: mprotect(2) - Set protection on a memory region
/// In no_std environment without MMU, just return success
/// Real implementation would modify page table entries
pub fn sys_mprotect(addr: usize, len: usize, prot: i32) -> isize {
    // Validate parameters
    if len == 0 {
        return 0; // Success for len=0
    }
    
    if addr % 4096 != 0 {
        return -22; // EINVAL - not page aligned
    }
    
    // Common protection flags
    const PROT_NONE: i32 = 0;
    const PROT_READ: i32 = 1;
    const PROT_WRITE: i32 = 2;
    const PROT_EXEC: i32 = 4;
    
    // Validate prot flags
    let valid_prot = prot == PROT_NONE 
        || (prot >= 0 && prot <= (PROT_READ | PROT_WRITE | PROT_EXEC));
    
    if !valid_prot {
        return -22; // EINVAL
    }
    
    // In a bare-metal/no_std environment without virtual memory,
    // we simply accept the request and return success
    // A full implementation would modify page table entries
    
    0
}

/// ioctl command constants
const FIONBIO: u32 = 0x5421;      // Set blocking mode (FIFO non-blocking)
const TCGETS: u32 = 0x5401;       // Get terminal attributes
const TCSETS: u32 = 0x5402;       // Set terminal attributes
const TIOCGWINSZ: u32 = 0x5413;   // Get window size
const TIOCSWINSZ: u32 = 0x5414;   // Set window size

/// Terminal attributes structure (simplified)
#[repr(C)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
}

/// Window size structure
#[repr(C)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

/// Syscall: ioctl(2) - Device-specific input/output control
pub fn sys_ioctl(fd: i32, cmd: u32, arg: usize) -> isize {
    // For stdin/stdout/stderr (fd 0,1,2), handle common ioctl commands
    match cmd {
        FIONBIO => {
            // Set non-blocking mode
            // arg is pointer to int (0 = blocking, 1 = non-blocking)
            // For now, just return success
            0
        }
        TCGETS => {
            // Get terminal attributes
            if fd == 0 || fd == 1 || fd == 2 {
                // For TTY, return default termios
                let termios = Termios {
                    c_iflag: 0x0300,
                    c_oflag: 0x0005,
                    c_cflag: 0xbf08,
                    c_lflag: 0x8a3b,
                    c_line: 0,
                    c_cc: [3, 28, 127, 21, 4, 1, 0, 0, 17, 19, 26, 0, 18, 15, 23, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                };
                
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &termios as *const _ as *const u8,
                        arg as *mut u8,
                        core::mem::size_of::<Termios>(),
                    );
                }
                0
            } else {
                -25 // ENOTTY
            }
        }
        TCSETS => {
            // Set terminal attributes - just return success
            0
        }
        TIOCGWINSZ => {
            // Get window size
            if fd == 0 || fd == 1 || fd == 2 {
                let winsize = Winsize {
                    ws_row: 24,
                    ws_col: 80,
                    ws_xpixel: 0,
                    ws_ypixel: 0,
                };
                
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &winsize as *const _ as *const u8,
                        arg as *mut u8,
                        core::mem::size_of::<Winsize>(),
                    );
                }
                0
            } else {
                -25 // ENOTTY
            }
        }
        TIOCSWINSZ => {
            // Set window size - just return success
            0
        }
        _ => {
            // Unknown command - return EINVAL for most cases
            // or ENOTTY for terminal-related commands
            if fd == 0 || fd == 1 || fd == 2 {
                -25 // ENOTTY
            } else {
                -25 // ENOTTY
            }
        }
    }
}
