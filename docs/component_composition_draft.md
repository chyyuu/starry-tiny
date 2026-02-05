# Starry-Tiny Component Composition Draft (arceos based)

Goal: combine existing arceos components into a macro-kernel that can run minimal Linux user programs.

## Layers and responsibilities
1) Boot and arch
- CPU init, trap/interrupts, context switch
- Syscall entry and return path (RISC-V64 ABI)

2) Task and process
- Task struct, scheduling, exit/exit_group handling
- Per-process fd table and address space

3) Memory management
- Virtual memory and page tables
- User memory copy helpers
- brk/mmap support (start with anon mmap)

4) VFS and FS
- VFS core and path resolution
- Simple FS backend (ramfs or in-memory fs image)
- openat/read/write/close/fstat/lseek

5) ELF loader
- Load user ELF from VFS
- Set up user stack, auxv, argv/envp

6) Device and console
- stdout/stderr to console
- block device if needed for fs image

## Component selection (draft)
- arceos task/process: scheduler + task exit
- arceos memory: vm + paging + user copy
- arceos fs: VFS + simple fs (ramfs or fs image)
- arceos elf: user ELF loader
- arceos drivers: console, optional block

## Integration steps
1. Build syscall dispatcher with Linux RISC-V64 ABI mapping.
2. Hook syscall layer to VFS and task components.
3. Bring up ELF loader using VFS read APIs.
4. Wire stdout/stderr to console driver.
5. Add fd table and per-process resources.

## Initial configuration target
- Single process, single address space
- No fork/exec yet (just exec on boot)
- Minimal signal handling (none or stub)
- Filesystem: static image or ramfs preloaded with test app

## References
- StarryOS syscall implementations as behavior baseline.
- tg-ch18 documents for syscall conventions and pitfalls.
