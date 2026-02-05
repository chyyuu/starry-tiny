use alloc::alloc::alloc;
use alloc::vec::Vec;
use core::alloc::Layout;
use super::ENOSYS;

static mut BRK_END: usize = 0;

/// Memory mapping tracker: (address, size)
#[allow(dead_code)]
static mut MMAP_REGIONS: Option<Vec<(usize, usize)>> = None;

#[allow(dead_code)]
const MAP_SHARED: usize = 0x01;
#[allow(dead_code)]
const MAP_PRIVATE: usize = 0x02;
const MAP_ANONYMOUS: usize = 0x20;
#[allow(dead_code)]
const PROT_READ: usize = 0x1;
#[allow(dead_code)]
const PROT_WRITE: usize = 0x2;

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

pub fn sys_mmap(_addr: usize, len: usize, _prot: usize, flags: usize, _fd: usize, _offset: usize) -> isize {
    // Only support anonymous mappings for now
    if (flags & MAP_ANONYMOUS) == 0 {
        return ENOSYS;
    }
    
    if len == 0 {
        return -22; // EINVAL
    }
    
    // Align length to page size (4096)
    let aligned_len = (len + 4095) & !4095;
    
    // Try to allocate memory
    let layout = match Layout::from_size_align(aligned_len, 4096) {
        Ok(l) => l,
        Err(_) => return -12, // ENOMEM
    };
    
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            return -12; // ENOMEM
        }
        
        // Initialize memory to zero
        core::ptr::write_bytes(ptr, 0, aligned_len);
        
        // Note: We skip allocation tracking due to mutable static restrictions
        // In production, use a SpinLock<Vec<>> or similar pattern
        
        ptr as isize
    }
}
