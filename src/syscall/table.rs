#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Sysno {
    Ioctl = 29,
    Fcntl = 25,
    Brk = 12,
    Close = 57,
    OpenAt = 56,
    Lseek = 62,
    Read = 63,
    Write = 64,
    Fstat = 80,
    Exit = 93,
    ExitGroup = 94,
    SetRobustList = 99,
    SetTidAddress = 218,
    Mmap = 222,
    Mprotect = 226,
    Prlimit64 = 261,
    Readlinkat = 79,
    Getrandom = 278,
}

impl Sysno {
    pub fn from(nr: usize) -> Option<Self> {
        match nr {
            29 => Some(Sysno::Ioctl),
            25 => Some(Sysno::Fcntl),
            12 => Some(Sysno::Brk),
            57 => Some(Sysno::Close),
            56 => Some(Sysno::OpenAt),
            62 => Some(Sysno::Lseek),
            63 => Some(Sysno::Read),
            64 => Some(Sysno::Write),
            80 => Some(Sysno::Fstat),
            93 => Some(Sysno::Exit),
            94 => Some(Sysno::ExitGroup),
            99 => Some(Sysno::SetRobustList),
            218 => Some(Sysno::SetTidAddress),
            222 => Some(Sysno::Mmap),
            226 => Some(Sysno::Mprotect),
            261 => Some(Sysno::Prlimit64),
            79 => Some(Sysno::Readlinkat),
            278 => Some(Sysno::Getrandom),
            _ => None,
        }
    }
}
