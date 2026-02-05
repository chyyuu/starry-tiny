# starry-tiny syscall 集成文档

## 已实现的 syscalls 及其对接 arceos 模块映射

根据 StarryOS 设计，以下 syscall 已通过对接 **StarryOS 的 arceos** 核心组件实现：

### 文件 I/O syscalls (fs.rs)

| Sysno | Name | 实现方式 | arceos API | 引入模块 |
|-------|------|--------|-----------|---------|
| 56 | openat | 支持 AT_FDCWD | `arceos_api::fs::ax_open_file` + `AxOpenOptions` | axfs |
| 63 | read | 用户指针 + arc eos fs | `arceos_api::fs::ax_read_file` | axfs |
| 64 | write | 用户指针 + arceos fs | `arceos_api::fs::ax_write_file` | axfs |
| 57 | close | fd 表管理 | SpinLock fd table | 本地 |
| 80 | fstat | 已实现，支持文件属性查询 | `ax_seek_file` 获取文件大小 | axfs/axio |
| 62 | lseek | 已实现，支持 Start/Current/End | `arceos_api::fs::ax_seek_file` + `AxSeekFrom` | axfs/axio |

### 进程控制 syscalls (task.rs)

| Sysno | Name | 实现方式 | arceos API |
|-------|------|--------|-----------|
| 93 | exit | 调用 ax_exit | `arceos_api::task::ax_exit` |
| 94 | exit_group | 同 exit（单进程模型） | `arceos_api::task::ax_exit` |

### 内存管理 syscalls (mm.rs)

| Sysno | Name | 实现方式 | 状态 |
|-------|------|--------|------|
| 12 | brk | 简单堆尾指针管理 | 最小实现，libc 可用 |
| 222 | mmap | stub | 待实现（需 mm 模块） |

### 线程/同步 syscalls (sync.rs, thread.rs)

| Sysno | Name | 实现方式 |
|-------|------|--------|
| 99 | set_robust_list | stub | 
| 218 | set_tid_address | stub |

---

## 关键对接设计

### 1. fd 表管理
**文件**: `src/syscall/fd_table.rs`

```rust
SpinLock<Vec<Option<AxFileHandle>>>
```
- `add_file(file) → i32`: 添加文件到 fd 表，返回 fd
- `with_file_mut(fd, fn) → Option<R>`: 对 fd 调用闭包函数
- `remove_file(fd) → Option<AxFileHandle>`: 移除 fd 并返回文件句柄

### 2. 错误码映射
**文件**: `src/syscall/mod.rs`

`AxError` (arceos) ↔ `LinuxError` ↔ `isize` (Linux syscall 返回值)

```rust
pub fn ax_err_to_isize(err: AxError) -> isize {
    -(LinuxError::from(err) as i32 as isize)
}
```

### 3. 用户指针访问
当前方式：直接 `unsafe` 指针读写（后续可升级到 VM 安全拷贝）

```rust
// 在 openat 中读取路径字符串
fn load_user_cstring(ptr: usize) -> AxResult<String>

// 在 fstat/read/write 中直接转换指针
let slice = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };
```

---

## 依赖关系与导入

| 源 | 导入 | 用途 |
|----|------|------|
| arceos_api::fs | AxOpenOptions, AxSeekFrom | 文件操作 |
| arceos_api::task | ax_exit | 进程退出 |
| axerrno | AxError, LinuxError | 错误号转换 |
| linux_raw_sys::general | stat, O_*, S_* | Linux 系统调用常数与数据结构 |

> **注意**: 当前项目对接的是 **StarryOS/arceos** 对应的 API，而非 os-compare/arceos。
> 这保证设计与 StarryOS 的 syscall 实现保持一致。

---

## 下一步迭代

1. **mmap 实现**: 需要学习 arceos axmm 模块，支持匿名映射
2. **用户态安全访问**: 迁移到 starry_vm/VmPtr 系列接口
3. **进程管理**: 实现 fork/execve 支持多进程
4. **信号处理**: 补充 signal/sigaction syscalls

