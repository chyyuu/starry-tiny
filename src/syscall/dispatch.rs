use super::{fs, mm, sync, table::Sysno, task, thread, ENOSYS};

pub fn handle_syscall(nr: usize, args: [usize; 6]) -> isize {
    match Sysno::from(nr) {
        Some(Sysno::OpenAt) => fs::sys_openat(args[0] as isize, args[1], args[2], args[3]),
        Some(Sysno::Read) => fs::sys_read(args[0] as i32, args[1], args[2]),
        Some(Sysno::Write) => fs::sys_write(args[0] as i32, args[1], args[2]),
        Some(Sysno::Close) => fs::sys_close(args[0] as i32),
        Some(Sysno::Fstat) => fs::sys_fstat(args[0] as i32, args[1]),
        Some(Sysno::Exit) => task::sys_exit(args[0] as i32),
        Some(Sysno::ExitGroup) => task::sys_exit_group(args[0] as i32),
        Some(Sysno::Brk) => mm::sys_brk(args[0]),
        Some(Sysno::Mmap) => mm::sys_mmap(args[0], args[1], args[2], args[3], args[4], args[5]),
        Some(Sysno::Lseek) => fs::sys_lseek(args[0] as i32, args[1] as isize, args[2] as i32),
        Some(Sysno::Fcntl) => fs::sys_fcntl(args[0] as i32, args[1] as i32, args[2]),
        Some(Sysno::SetRobustList) => sync::sys_set_robust_list(args[0], args[1]),
        Some(Sysno::SetTidAddress) => thread::sys_set_tid_address(args[0]),
        None => ENOSYS,
    }
}
