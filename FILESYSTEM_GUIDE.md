# starry-tiny 文件系统使用指南

## 概述

本指南说明如何为starry-tiny创建包含应用程序的ext4磁盘镜像，以及如何在StarryOS或其他操作系统中使用它。

## 文件结构

```
starry-tiny/
├── Makefile                      # 磁盘镜像管理规则
├── disk.img                      # 生成的ext4磁盘镜像（编译后）
├── scripts/
│   └── create-disk-image.sh      # 磁盘镜像创建脚本
└── linux-app/
    ├── ch18_file0.c              # 测试程序源代码
    └── ch18_file0                # 编译后的RISC-V64可执行文件
```

## 快速开始

### 1. 编译应用程序

首先，编译ch18_file0为RISC-V64可执行文件：

```bash
cd /home/chyyuu/thecodes/os-compare/starry-tiny

# 使用RISC-V工具链编译
make compile-apps
```

**前提条件**：需要安装RISC-V工具链
```bash
# Ubuntu/Debian
sudo apt-get install gcc-riscv64-unknown-elf

# 或从GitHub下载预编译版本
# https://github.com/riscv-collab/riscv-gnu-toolchain/releases
```

### 2. 创建磁盘镜像

创建ext4磁盘镜像并将应用程序添加到其中：

```bash
make disk-image
```

这个命令会：
- 创建256MB的ext4磁盘镜像 (`disk.img`)
- 挂载镜像到 `/tmp/starry_tiny_mnt`
- 编译并复制 `ch18_file0` 到镜像中
- 创建初始化脚本 `/init`
- 卸载镜像

**输出示例**：
```
[INFO] Creating disk image for starry-tiny
[INFO] Checking requirements...
[INFO] Creating ext4 disk image (256MB)...
[INFO] Disk image created: /home/chyyuu/thecodes/os-compare/starry-tiny/disk.img
[INFO] Mounting disk image...
[INFO] Compiling ch18_file0...
[INFO] Adding ch18_file0 to disk image...
[INFO] Creating init script...
[INFO] Done! Disk image ready: disk.img
```

### 3. 验证磁盘镜像

```bash
# 查看镜像信息
make disk-info

# 输出示例
# [INFO] Disk Image Info
# Disk image: disk.img
# -rw-r--r-- 1 user user 256M Feb  5 10:30 disk.img
# disk.img: Linux ext4 filesystem data (Ext4, version 1.0)
# Mount point: /tmp/starry_tiny_mnt
# Status: Not mounted
#
# Contents (when mounted):
# total 100K
# -rwxr-xr-x root root 21K ch18_file0
# -rwxr-xr-x root root  46B init
```

## 使用集成到StarryOS

### 方式1: 替换StarryOS的disk.img

```bash
# 1. 创建starry-tiny的磁盘镜像
cd /home/chyyuu/thecodes/os-compare/starry-tiny
make disk-image

# 2. 备份并替换StarryOS的disk.img
cd /home/chyyuu/thecodes/os-compare/StarryOS/arceos
cp disk.img disk.img.bak
cp ../starry-tiny/disk.img disk.img

# 3. 运行StarryOS
make run ARCH=riscv64
```

### 方式2: 在运行时挂载并修改

```bash
# 1. 创建初始磁盘镜像
make disk-image

# 2. 挂载磁盘镜像
make disk-mount

# 3. 添加更多应用程序（可选）
make disk-add-app APP=ch18_file1

# 4. 查看磁盘内容
make disk-info

# 5. 卸载磁盘镜像
make disk-unmount
```

## 磁盘镜像管理命令

### 创建和删除

```bash
# 创建磁盘镜像
make disk-image

# 删除磁盘镜像
make disk-clean
```

### 挂载和卸载

```bash
# 挂载磁盘镜像
make disk-mount

# 查看磁盘内容（需要先挂载）
ls -la /tmp/starry_tiny_mnt/

# 卸载磁盘镜像
make disk-unmount
```

### 添加应用程序

```bash
# 先创建并编译应用
cd linux-app
vim ch18_file1.c          # 创建新程序

# 回到项目根目录
cd ..

# 编译
make compile-apps

# 挂载磁盘并添加
make disk-mount
make disk-add-app APP=ch18_file1
make disk-unmount
```

### 查看信息

```bash
# 显示所有可用命令
make disk-help

# 查看磁盘镜像信息
make disk-info
```

## 磁盘镜像结构

创建的磁盘镜像包含以下文件：

```
/ (ext4 filesystem)
├── init              # 初始化脚本（可选）
├── ch18_file0        # 文件I/O测试程序
├── ch18_file1        # （可选）
└── ...其他应用
```

### init脚本

默认创建的`/init`脚本：

```bash
#!/bin/sh
echo "Starting starry-tiny filesystem..."
exec /ch18_file0
```

可根据需要修改为运行不同的应用程序。

## 添加自己的应用程序

### 1. 编写C源代码

```bash
# linux-app/my_app.c
cat > linux-app/my_app.c << 'EOF'
#include <stdio.h>
#include <unistd.h>

int main() {
    printf("Hello from my_app!\n");
    return 0;
}
EOF
```

### 2. 编译应用

```bash
# 自动编译所有C程序
make compile-apps

# 或手动编译（如果有交叉编译器）
riscv64-unknown-linux-gnu-gcc -static -O2 -o linux-app/my_app linux-app/my_app.c
```

### 3. 添加到磁盘

```bash
# 方式1: 重新创建整个磁盘镜像
make disk-image

# 方式2: 以现有镜像为基础添加
make disk-mount
make disk-add-app APP=my_app
make disk-unmount
```

## 故障排除

### 问题1: "mkfs.ext4" 未找到

**解决**：安装 `e2fsprogs`
```bash
sudo apt-get install e2fsprogs
```

### 问题2: 权限拒绝 (Permission denied)

**解决**：脚本需要 `sudo` 权限来挂载磁盘。确保：
- 用户在 `sudo` 组中
- 或使用 `sudo` 明确运行：`sudo make disk-image`

### 问题3: 特定mount point已被使用

**解决**：卸载并清理
```bash
# 强制卸载（小心！不确定时不要用）
sudo umount -l /tmp/starry_tiny_mnt

# 或使用脚本提供的卸载命令
make disk-unmount
```

### 问题4: RISC-V编译器未找到

**解决**：安装RISC-V GNU工具链
```bash
# Ubuntu官方源
sudo apt-get install gcc-riscv64-unknown-elf

# 或从GitHub下载最新版
# https://github.com/riscv-collab/riscv-gnu-toolchain/releases
```

## 与StarryOS集成

### starry-tiny库的角色

- **syscall层**：实现Linux系统调用的标准接口
- **文件系统**：通过disk.img装载应用文件系统
- **执行环境**：由StarryOS kernel提供，使用starry-tiny的syscall

### 工作流

```
StarryOS kernel
      ↓
syscall分发器 (dispatch.rs)
      ↓
starry-tiny syscall库 (fs.rs, mm.rs等)
      ↓
arceos modules (axfs, axtask等)
      ↓
disk.img 文件系统
      ↓
应用程序 (ch18_file0)
```

## 参考资源

- [StarryOS 主项目](https://github.com/Starry-OS/Starry)
- [Linux ext4 文件系统](https://ext4.wiki.kernel.org/)
- [RISC-V GNU 工具链](https://github.com/riscv-collab/riscv-gnu-toolchain)

## 许可证

本项目遵循与starry-tiny相同的许可证 (Apache 2.0)。
