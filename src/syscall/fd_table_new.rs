/// Stub file descriptor table (without arceos integration)
/// 
/// This is a minimal implementation that doesn't actually manage files
/// until we have arceos integration.

pub struct FdTable;

impl FdTable {
    pub const fn new() -> Self {
        Self
    }
}

pub fn add_file(_file: ()) -> i32 {
    -1
}

pub fn with_file_mut<F, R>(_fd: i32, _f: F) -> Option<R>
where
    F: FnOnce(&mut ()) -> R,
{
    None
}

pub fn remove_file(_fd: i32) -> Option<()> {
    None
}
