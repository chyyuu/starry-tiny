#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Sysno {
    OpenAt = 56,
    Read = 63,
    Write = 64,
    Close = 57,
    Fstat = 80,
    Exit = 93,
    ExitGroup = 94,
    Brk = 12,
    Mmap = 222,
    Lseek = 62,
    Fcntl = 25,
    SetRobustList = 99,
    SetTidAddress = 218,
}

impl Sysno {
    pub fn from(nr: usize) -> Option<Self> {
        match nr {
            56 => Some(Sysno::OpenAt),
            63 => Some(Sysno::Read),
            64 => Some(Sysno::Write),
            57 => Some(Sysno::Close),
            80 => Some(Sysno::Fstat),
            93 => Some(Sysno::Exit),
            94 => Some(Sysno::ExitGroup),
            12 => Some(Sysno::Brk),
            222 => Some(Sysno::Mmap),
            62 => Some(Sysno::Lseek),
            25 => Some(Sysno::Fcntl),
            99 => Some(Sysno::SetRobustList),
            218 => Some(Sysno::SetTidAddress),
            _ => None,
        }
    }
}
