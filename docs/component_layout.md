# Starry-Tiny Concrete Layout (Draft)

This layout turns the composition draft into a concrete directory plan.

## Proposed tree

starry-tiny/
  docs/
  linux-app/
  src/
    lib.rs
    syscall/
      dispatch.rs
      fs.rs
      mm.rs
      mod.rs
      sync.rs
      table.rs
      task.rs
      thread.rs
  kernel/
    arch/
    boot/
    trap/
  mm/
  task/
  fs/
  elf/
  drivers/
  platform/
  tools/

## Mapping to responsibilities
- kernel/arch, kernel/trap: syscall entry, trap handling, context switch
- task/: process and scheduler, fd table, exit/exit_group
- mm/: vm, paging, user copy, brk/mmap
- fs/: vfs, path resolve, openat/read/write/close
- elf/: user ELF loader, stack and auxv
- drivers/: console and block device backends

## Integration sequence
1. Build syscall entry in kernel/trap and call src/syscall/dispatch.rs
2. Route syscalls into fs/mm/task modules
3. Use fs to read user ELF and load via elf module
4. Wire console for stdout/stderr
5. Add minimal filesystem image loader

## Configuration knobs (draft)
- FS backend: ramfs or image-backed
- MM: anon mmap only vs file-backed
- Task model: single process vs fork/exec later
- Console: UART or host console
