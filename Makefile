# Makefile for starry-tiny filesystem and disk image

.PHONY: disk-image disk-clean disk-mount disk-unmount disk-info

# Disk image location
DISK_IMG := disk.img
MOUNT_POINT ?= /tmp/starry_tiny_mnt

# Colors
BLUE := \033[0;34m
GREEN := \033[0;32m
RED := \033[0;31m
NC := \033[0m

# Create disk image with filesystem and applications
disk-image:
	@echo "$(BLUE)==== Creating Disk Image ====$(NC)"
	@chmod +x scripts/create-disk-image.sh
	@scripts/create-disk-image.sh
	@echo "$(GREEN)✓ Disk image created$(NC)"

# Clean disk image
disk-clean:
	@echo "$(BLUE)==== Cleaning Disk Image ====$(NC)"
	@if [ -f "$(DISK_IMG)" ]; then \
		rm -f "$(DISK_IMG)"; \
		echo "$(GREEN)✓ Disk image removed$(NC)"; \
	else \
		echo "$(RED)✗ No disk image found$(NC)"; \
	fi

# Mount disk image
disk-mount:
	@echo "$(BLUE)==== Mounting Disk Image ====$(NC)"
	@if [ ! -f "$(DISK_IMG)" ]; then \
		echo "$(RED)✗ Disk image not found. Run 'make disk-image' first.$(NC)"; \
		exit 1; \
	fi
	@mkdir -p "$(MOUNT_POINT)"
	@sudo mount -o loop "$(DISK_IMG)" "$(MOUNT_POINT)" && \
		echo "$(GREEN)✓ Disk mounted at $(MOUNT_POINT)$(NC)" || \
		echo "$(RED)✗ Failed to mount disk$(NC)"

# Unmount disk image
disk-unmount:
	@echo "$(BLUE)==== Unmounting Disk Image ====$(NC)"
	@if mount | grep -q "$(MOUNT_POINT)"; then \
		sudo umount "$(MOUNT_POINT)" && \
		echo "$(GREEN)✓ Disk unmounted$(NC)" || \
		echo "$(RED)✗ Failed to unmount disk$(NC)"; \
	else \
		echo "$(RED)✗ Disk not mounted$(NC)"; \
	fi

# Show disk image info
disk-info:
	@echo "$(BLUE)==== Disk Image Info ====$(NC)"
	@if [ -f "$(DISK_IMG)" ]; then \
		echo "Disk image: $(DISK_IMG)"; \
		ls -lh "$(DISK_IMG)"; \
		file "$(DISK_IMG)"; \
	else \
		echo "$(RED)✗ Disk image not found$(NC)"; \
	fi
	@echo ""
	@echo "$(BLUE)Mount point: $(MOUNT_POINT)$(NC)"
	@if mount | grep -q "$(MOUNT_POINT)"; then \
		echo "$(GREEN)Status: Mounted$(NC)"; \
		echo "$(BLUE)Contents:$(NC)"; \
		sudo ls -lh "$(MOUNT_POINT)/"; \
	else \
		echo "$(RED)Status: Not mounted$(NC)"; \
	fi

# Add application to mounted disk
disk-add-app:
	@if [ -z "$(APP)" ]; then \
		echo "$(RED)✗ Usage: make disk-add-app APP=<app_name>$(NC)"; \
		exit 1; \
	fi
	@if ! mount | grep -q "$(MOUNT_POINT)"; then \
		echo "$(RED)✗ Disk not mounted. Run 'make disk-mount' first.$(NC)"; \
		exit 1; \
	fi
	@if [ ! -f "linux-app/$(APP)" ]; then \
		echo "$(RED)✗ Application not found: linux-app/$(APP)$(NC)"; \
		exit 1; \
	fi
	@echo "$(BLUE)Adding $(APP) to disk image...$(NC)"
	@sudo cp "linux-app/$(APP)" "$(MOUNT_POINT)/$(APP)"
	@sudo chmod +x "$(MOUNT_POINT)/$(APP)"
	@echo "$(GREEN)✓ Added $(APP)$(NC)"

# Compile applications for starry-tiny
.PHONY: compile-apps
compile-apps:
	@echo "$(BLUE)==== Compiling Applications ====$(NC)"
	@if command -v riscv64-unknown-linux-gnu-gcc >/dev/null 2>&1; then \
		echo "Using riscv64-unknown-linux-gnu-gcc"; \
		cd linux-app && \
		for file in *.c; do \
			if [ -f "$$file" ]; then \
				app=$${file%.c}; \
				echo "Compiling $$app..."; \
				riscv64-unknown-linux-gnu-gcc -static -O2 -o $$app $$file || exit 1; \
			fi; \
		done; \
	else \
		echo "$(RED)✗ riscv64-unknown-linux-gnu-gcc not found$(NC)"; \
		echo "Install RISC-V toolchain: apt install gcc-riscv64-unknown-elf"; \
		exit 1; \
	fi
	@echo "$(GREEN)✓ Compilation complete$(NC)"

# Help target
disk-help:
	@echo "$(BLUE)=== Disk Image Management ===$(NC)"
	@echo ""
	@echo "$(GREEN)Available targets:$(NC)"
	@echo "  make disk-image       - Create disk image with filesystem"
	@echo "  make disk-mount       - Mount disk image at $(MOUNT_POINT)"
	@echo "  make disk-unmount     - Unmount disk image"
	@echo "  make disk-info        - Show disk image information"
	@echo "  make disk-clean       - Remove disk image"
	@echo "  make disk-add-app APP=<name> - Add application to disk"
	@echo "  make compile-apps     - Compile all C applications for RISC-V"
	@echo ""
	@echo "$(BLUE)Example workflow:$(NC)"
	@echo "  1. make compile-apps      # Compile applications"
	@echo "  2. make disk-image        # Create disk image with apps"
	@echo "  3. make disk-info         # Verify disk contents"
	@echo ""
