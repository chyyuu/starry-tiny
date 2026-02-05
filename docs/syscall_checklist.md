# Starry-Tiny Syscall Checklist

Purpose: track Linux RISC-V64 syscall conventions and how they map to arceos components. Use this before implementing any syscall.

## Checklist Columns
- Sysno: Linux RISC-V64 syscall number
- Name: syscall name
- Args: argument list and order
- Special handling: e.g., must not return, or uses AT_FDCWD
- arceos mapping: component or API used to implement
- Status: todo | wip | done | blocked
- Notes: pitfalls and references

## Core Checklist (seed)
| Sysno | Name | Args | Special handling | arceos mapping | Status | Notes |
|---|---|---|---|---|---|---|
| 56 | openat | dirfd, path*, flags, mode | dirfd == -100 means CWD | VFS + path resolve | todo | Use 4 args, not 3 |
| 63 | read | fd, buf*, count | copy_to_user | VFS read | todo | return bytes read |
| 64 | write | fd, buf*, count | copy_from_user | VFS write | todo | stdout/stderr via console |
| 57 | close | fd | release fd | fd table | todo | handle invalid fd |
| 80 | fstat | fd, statbuf* | fill struct stat | VFS stat | todo | glibc uses this often |
| 93 | exit | code | must not return | task exit | todo | terminate current task |
| 94 | exit_group | code | must not return | task exit | todo | terminate process group |
| 12 | brk | addr | heap end | mm/heap | todo | needed by libc |
| 222 | mmap | addr, len, prot, flags, fd, off | anon ok | mm/vm | todo | start with anon only |

## Optional / libc support
| Sysno | Name | Args | Special handling | arceos mapping | Status | Notes |
|---|---|---|---|---|---|---|
| 62 | lseek | fd, offset, whence | return new offset | VFS seek | todo | some libc use |
| 25 | fcntl | fd, cmd, arg | cmd specific | fd ops | todo | minimal cmds first |
| 214 | brk | addr | already listed above | mm/heap | todo | verify sysno if needed |
| 99 | set_robust_list | head*, len | return 0 | task/thread | todo | glibc expects 0 |
| 218 | set_tid_address | tidp* | return tid | task/thread | todo | glibc expects value |

## Usage Rules
- Verify args and order against Linux RISC-V64 tables before coding.
- Check StarryOS syscall implementation for reference behavior.
- If a syscall must not return, enforce in dispatcher (not just handler).
- Record any deviations or temporary stubs in Notes.
