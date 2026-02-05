pub mod dispatch;
pub mod fd_table;
pub mod fs;
pub mod mm;
pub mod sync;
pub mod table;
pub mod task;
pub mod thread;

pub use dispatch::handle_syscall;

pub const ENOSYS: isize = -38;

use axerrno::{AxError, LinuxError};

pub fn ax_err_to_isize(err: AxError) -> isize {
	-(LinuxError::from(err) as i32 as isize)
}

pub fn linux_err_to_isize(err: LinuxError) -> isize {
	-(err as i32 as isize)
}
