#!/bin/bash
# Create ext4 disk image for starry-tiny and add applications

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DISK_IMG="$PROJECT_ROOT/disk.img"
MOUNT_POINT="/tmp/starry_tiny_mnt"

# Configuration
DISK_SIZE=256  # MB
APPS=("ch18_file0")

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

check_requirements() {
    log_info "Checking requirements..."
    
    # Check for required tools
    for cmd in dd mkfs.ext4 sudo mount umount; do
        if ! command -v $cmd &> /dev/null; then
            log_error "$cmd is not installed. Please install it first."
        fi
    done
    
    log_info "All requirements met."
}

create_disk_image() {
    log_info "Creating ext4 disk image (${DISK_SIZE}MB)..."
    
    if [ -f "$DISK_IMG" ]; then
        log_warn "Disk image already exists: $DISK_IMG"
        read -p "Overwrite? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Keeping existing disk image."
            return 0
        fi
        rm -f "$DISK_IMG"
    fi
    
    # Create empty file
    dd if=/dev/zero of="$DISK_IMG" bs=1M count=$DISK_SIZE 2>/dev/null
    
    # Format as ext4
    mkfs.ext4 -F "$DISK_IMG" > /dev/null 2>&1
    
    log_info "Disk image created: $DISK_IMG"
}

mount_disk_image() {
    log_info "Mounting disk image..."
    
    # Create mount point
    mkdir -p "$MOUNT_POINT"
    
    # Check if already mounted
    if mount | grep -q "$MOUNT_POINT"; then
        log_warn "Mount point already in use. Unmounting first..."
        sudo umount "$MOUNT_POINT" || true
    fi
    
    # Mount
    sudo mount -o loop "$DISK_IMG" "$MOUNT_POINT"
    
    log_info "Disk mounted at: $MOUNT_POINT"
}

unmount_disk_image() {
    log_info "Unmounting disk image..."
    
    if mount | grep -q "$MOUNT_POINT"; then
        sudo umount "$MOUNT_POINT"
        log_info "Disk unmounted."
    else
        log_warn "Mount point not in use."
    fi
}

compile_app() {
    local app_name=$1
    local source_dir="$PROJECT_ROOT/linux-app"
    local source_file="$source_dir/${app_name}.c"
    local output_file="$source_dir/${app_name}"
    
    # Check if precompiled version exists from tg-ch18
    local precompiled="/home/chyyuu/thecodes/os-compare/tg-ch18/linux-compatible-tests/$app_name"
    if [ -f "$precompiled" ]; then
        log_info "Found precompiled version: $precompiled"
        cp "$precompiled" "$output_file"
        log_info "$app_name ready: $output_file"
        return 0
    fi
    
    if [ ! -f "$source_file" ]; then
        log_warn "Source file not found: $source_file"
        return 1
    fi
    
    log_info "Compiling $app_name..."
    
    # Try to compile as riscv64 if cross-compiler available
    if command -v riscv64-unknown-linux-gnu-gcc &> /dev/null; then
        log_info "Using riscv64-unknown-linux-gnu-gcc"
        riscv64-unknown-linux-gnu-gcc -static -O2 \
            "$source_file" -o "$output_file" \
            || log_error "Compilation failed for $app_name"
    elif command -v riscv64-linux-gnu-gcc &> /dev/null; then
        log_info "Using riscv64-linux-gnu-gcc"
        riscv64-linux-gnu-gcc -static -O2 \
            "$source_file" -o "$output_file" \
            || log_error "Compilation failed for $app_name"
    elif command -v gcc &> /dev/null; then
        log_warn "riscv64 cross-compiler not found, using native gcc"
        gcc -static -O2 \
            "$source_file" -o "$output_file" \
            || log_error "Compilation failed for $app_name"
    else
        log_error "No suitable C compiler found"
    fi
    
    log_info "$app_name compiled: $output_file"
}

add_app_to_disk() {
    local app_name=$1
    local app_path="$PROJECT_ROOT/linux-app/$app_name"
    
    if [ ! -f "$app_path" ]; then
        log_warn "Application not found: $app_path"
        return 1
    fi
    
    log_info "Adding $app_name to disk image..."
    
    # Copy executable
    sudo cp "$app_path" "$MOUNT_POINT/$app_name"
    sudo chmod +x "$MOUNT_POINT/$app_name"
    
    log_info "Added: /$app_name"
}

add_init_script() {
    log_info "Creating init script..."
    
    # Create a simple init script that executes ch18_file0
    cat > /tmp/init.sh << 'EOF'
#!/bin/sh
# Simple init script for starry-tiny
echo "Starting starry-tiny filesystem..."
exec /ch18_file0
EOF
    
    sudo cp /tmp/init.sh "$MOUNT_POINT/init"
    sudo chmod +x "$MOUNT_POINT/init"
    rm /tmp/init.sh
    
    log_info "Init script created: /init"
}

main() {
    log_info "Creating disk image for starry-tiny"
    log_info "=================================="
    
    check_requirements
    create_disk_image
    mount_disk_image
    
    # Compile all applications
    for app in "${APPS[@]}"; do
        compile_app "$app" || true
    done
    
    # Add applications to disk
    for app in "${APPS[@]}"; do
        add_app_to_disk "$app" || true
    done
    
    # Create init script
    add_init_script
    
    # List disk contents
    log_info "Disk image contents:"
    sudo ls -lh "$MOUNT_POINT/"
    
    unmount_disk_image
    
    log_info "=================================="
    log_info "Done! Disk image ready: $DISK_IMG"
}

# Run main function
main "$@"
