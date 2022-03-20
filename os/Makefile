# 期望功能: run 利用qemu运行
# debug 开启qemu debug模式
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ENTRY_PA := 0x80200000
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOARD := qemu
SBI := rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
OBJCOPY := rust-objcopy --binary-architecture=riscv64

kernel:
	@cd ../user && make build
ifeq ($(MODE),debug)
	@cargo build 
else
	@cargo build --release
endif
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

run: kernel
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

debug: kernel
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S

.PHONY: kernel run debug