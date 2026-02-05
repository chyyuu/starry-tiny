/// Stub file descriptor table
/// 
/// A minimal implementation for the fd_table module until arceos integration is complete.
/// Currently returns errors for file operations.

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
