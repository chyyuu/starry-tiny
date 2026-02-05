# Minimal Syscalls for linux-app/ch18_file0.c

This list targets the program in [linux-app/ch18_file0.c](linux-app/ch18_file0.c).

## Tier 1: Required by program logic
- openat (sysno 56): used via libc open() with AT_FDCWD
- write (sysno 64): used by write() and printf
- read (sysno 63): used by read()
- close (sysno 57): used by close()
- exit_group (sysno 94): used by libc on exit

## Tier 2: Likely required by libc runtime
These are commonly used by glibc even for simple programs.
- brk (sysno 12): heap management
- mmap (sysno 222): memory allocation or TLS
- fstat (sysno 80): stdio initialization
- lseek (sysno 62): sometimes used by stdio
- fcntl (sysno 25): fd flags (minimal cmds)
- set_tid_address (sysno 218): thread bookkeeping
- set_robust_list (sysno 99): thread cleanup list

## Minimal behavior notes
- openat: must accept 4 args; dirfd == -100 means CWD.
- exit_group: must terminate task/process and not return.
- errno: return negative errno values as Linux does.
- user pointers: validate or safely copy to/from user memory.

## Validation
- Expected success output: "Test file0 OK!"
- Failure should print perror message and exit non-zero.
