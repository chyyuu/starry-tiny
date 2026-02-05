pub mod dispatch;
pub mod fs;
pub mod mm;
pub mod sync;
pub mod table;
pub mod task;
pub mod thread;

pub use dispatch::handle_syscall;

pub const ENOSYS: isize = -38;
