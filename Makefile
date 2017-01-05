# Directories
BUILD_DIR=build
TARGET_DIR=target/x86_64-unknown-none-gnu/debug
PREFIX=tools/bin

# Tools
MAKE_ISO=tools/bin/grub-mkrescue
QEMU=qemu-system-x86_64
CARGO=xargo
LD=$(PREFIX)/x86_64-elf-ld

.PHONY: all run clean

all: osdev.iso

$(TARGET_DIR)/libkernel.a: kernel/**
	$(CARGO) build --target=x86_64-unknown-none-gnu --package kernel

$(TARGET_DIR)/osdev.bin: $(TARGET_DIR)/libkernel.a
	$(LD) --gc-sections -T kernel/src/x86_64/linker.ld  $(TARGET_DIR)/libkernel.a -o $(TARGET_DIR)/osdev.bin

osdev.iso: $(TARGET_DIR)/osdev.bin
	mkdir -p $(TARGET_DIR)/isodir/boot/grub
	cp $(TARGET_DIR)/osdev.bin $(TARGET_DIR)/isodir/boot
	cp data/grub.cfg $(TARGET_DIR)/isodir/boot/grub
	$(MAKE_ISO) -o osdev.iso $(TARGET_DIR)/isodir

run:
	$(QEMU) -cdrom osdev.iso
