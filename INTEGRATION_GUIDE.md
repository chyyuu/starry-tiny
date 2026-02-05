# 将 starry-tiny 磁盘镜像集成到 StarryOS

本文档说明如何将starry-tiny创建的disk.img集成到StarryOS中运行。

## 前置条件

- StarryOS项目已克隆
- starry-tiny已创建disk.img（通过`make disk-image`）
- riscv64架构的qemu已安装

## 集成步骤

### 方式1: 直接替换（推荐）

#### 1.1 备份原始镜像

```bash
cd /home/chyyuu/thecodes/os-compare/StarryOS/arceos
cp disk.img disk.img.original
```

#### 1.2 复制starry-tiny的磁盘镜像

```bash
cp /home/chyyuu/thecodes/os-compare/starry-tiny/disk.img disk.img
```

#### 1.3 验证镜像

```bash
# 查看镜像信息
ls -lh disk.img
file disk.img

# 输出应类似：
# -rw-r--r-- 1 user user 256M Feb  5 21:35 disk.img
# disk.img: Linux rev 1.0 ext4 filesystem data
```

#### 1.4 在StarryOS中运行

```bash
# RISC-V64架构
make run ARCH=riscv64

# 预期输出：
# [开始运行StarryOS虚拟机]
# [加载disk.img文件系统]
# [执行ch18_file0程序]
# Test file0 OK!
```

### 方式2: 编译时集成

如果想在StarryOS编译过程中自动使用disk.img：

#### 2.1 修改StarryOS Makefile（如需要）

```makefile
# StarryOS/Makefile 或 arceos/Makefile
# 某些版本可能需要显式指定disk_img位置
DISK_IMG ?= disk.img
```

#### 2.2 放置域名镜像

```bash
cp /home/chyyuu/thecodes/os-compare/starry-tiny/disk.img \
   /home/chyyuu/thecodes/os-compare/StarryOS/arceos/disk.img
```

#### 2.3 构建并运行

```bash
cd /home/chyyuu/thecodes/os-compare/StarryOS
make build ARCH=riscv64
make run ARCH=riscv64
```

## 验证执行

### 预期输出
当StarryOS成功加载starry-tiny的disk.img并执行ch18_file0时，应该看到：

```
[ ...timestamp... ] StarryOS starting...
[ ...timestamp... ] Initialize filesystem subsystem...
[ ...timestamp... ] Mounting filesystem from disk.img...
[ ...timestamp... ] Syscall brk return Ok(...)
[ ...timestamp... ] Syscall set_tid_address return Ok(...)
[ ...timestamp... ] Syscall prlimit64 return Ok(0)
[ ...timestamp... ] Syscall openat return Ok(3)
[ ...timestamp... ] Syscall write return Ok(13)
[ ...timestamp... ] Syscall read return Ok(13)
[ ...timestamp... ] Syscall write return Ok(15)
Test file0 OK!
```

### 测试项目

ch18_file0包含以下功能测试：

1. **文件创建**：`open(fname, O_CREAT | O_WRONLY)`
2. **文件写入**：向文件写入 "Hello, world!"
3. **文件关闭**：正确关闭文件描述符
4. **文件读取**：重新打开并读取文件内容
5. **内容验证**：验证读取内容与写入内容一致

## 故障排除

### 问题1: "disk.img not found" 错误

**症状**：
```
Error: disk.img not found
```

**解决**：
```bash
# 确认文件存在
ls -lh /home/chyyuu/thecodes/os-compare/StarryOS/arceos/disk.img

# 如果不存在，复制它
cp /home/chyyuu/thecodes/os-compare/starry-tiny/disk.img \
   /home/chyyuu/thecodes/os-compare/StarryOS/arceos/disk.img
```

### 问题2: ch18_file0未执行

**症状**：
StarryOS启动但ch18_file0程序未运行

**原因和解决**：
```bash
# 1. 检查磁盘镜像是否正确挂载
make disk-mount
ls -la /tmp/starry_tiny_mnt/

# 应该看到 ch18_file0 和 init

# 2. 检查文件权限
sudo ls -la /tmp/starry_tiny_mnt/ch18_file0
# 应该是可执行的 (x权限)

# 3. 检查磁盘镜像完整性
file /home/chyyuu/thecodes/os-compare/StarryOS/arceos/disk.img
# 应该显示 "Linux rev 1.0 ext4 filesystem data"

make disk-unmount
```

### 问题3: "Permission denied" 错误

**症状**：
```
Permission denied while executing ch18_file0
```

**解决**：
```bash
# 重新创建磁盘镜像以确保权限正确
cd /home/chyyuu/thecodes/os-compare/starry-tiny
make disk-clean
make disk-image
```

### 问题4: 文件系统损坏

**症状**：
```
filesystem corruption detected
read-only filesystem
```

**解决**：
```bash
# 检查磁盘完整性
fsck.ext4 -n disk.img

# 如果有问题，重新创建
make disk-clean
make disk-image
```

## 修改磁盘镜像内容

### 添加新应用

```bash
cd /home/chyyuu/thecodes/os-compare/starry-tiny

# 1. 编写新程序
cat > linux-app/myapp.c << 'EOF'
#include <stdio.h>

int main() {
    printf("Hello from myapp!\n");
    return 0;
}
EOF

# 2. 编译
make compile-apps

# 3. 添加到磁盘
make disk-mount
make disk-add-app APP=myapp
make disk-unmount

# 4. 复制到StarryOS
cp disk.img /home/chyyuu/thecodes/os-compare/StarryOS/arceos/disk.img
```

### 修改init脚本

```bash
# 1. 挂载
make disk-mount

# 2. 编辑init脚本
sudo vi /tmp/starry_tiny_mnt/init

# 3. 卸载
make disk-unmount
```

## 性能考虑

- **磁盘镜像大小**：256MB（可在scripts/create-disk-image.sh中调整）
- **执行速度**：由qemu虚拟化性能决定
- **内存使用**：StarryOS + 磁盘缓存

## 相关资源

- [StarryOS项目](https://github.com/Starry-OS/Starry)
- [ch18_file0测试程序](../linux-app/ch18_file0.c)
- [文件系统管理](./FILESYSTEM_GUIDE.md)

## 注意事项

1. **备份重要数据**：修改disk.img前务必备份
2. **权限问题**：某些操作需要sudo权限
3. **版本兼容性**：确保StarryOS和starry-tiny版本相匹配
4. **架构选择**：当前配置针对riscv64，x86_64需要重新编译

## 常用命令速查表

```bash
# 创建磁盘
make disk-image

# 与StarryOS集成
cp disk.img ../StarryOS/arceos/disk.img

# 在StarryOS中运行
cd ../StarryOS && make run ARCH=riscv64

# 监控磁盘状态
make disk-info

# 清理
make disk-clean

# 查看所有命令
make disk-help
```
