# Directories
PREFIX=tools/bin
TARGET_DIR=target/x86_64-unknown-none-gnu/debug
OUT_DIR=$(TARGET_DIR)/build
ARCH_DIR=kernel/src/x86_64

# Source & output files
ASM_SOURCE_FILES := $(wildcard $(ARCH_DIR)/*.asm)
ASM_OUT_FILES := $(patsubst $(ARCH_DIR)/%.asm, $(OUT_DIR)/%.o, $(ASM_SOURCE_FILES))

# Tools
MAKE_ISO=tools/bin/grub-mkrescue
QEMU=qemu-system-x86_64
CARGO=xargo
LD=$(PREFIX)/x86_64-elf-ld

.PHONY: all run clean

all: osdev.iso

iso: osdev.iso

$(TARGET_DIR)/libkernel.a: kernel/**
	$(CARGO) build --target=x86_64-unknown-none-gnu --package kernel

$(TARGET_DIR)/osdev.bin: $(ARCH_DIR)/linker.ld $(TARGET_DIR)/libkernel.a $(ASM_OUT_FILES)
	$(LD) --gc-sections --nmagic -T $(ARCH_DIR)/linker.ld -o $(TARGET_DIR)/osdev.bin \
			$(TARGET_DIR)/libkernel.a $(ASM_OUT_FILES)

$(OUT_DIR)/%.o: $(ARCH_DIR)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

osdev.iso: $(TARGET_DIR)/osdev.bin
	mkdir -p $(TARGET_DIR)/isodir/boot/grub
	cp $(TARGET_DIR)/osdev.bin $(TARGET_DIR)/isodir/boot
	cp data/grub.cfg $(TARGET_DIR)/isodir/boot/grub
	$(MAKE_ISO) -o osdev.iso $(TARGET_DIR)/isodir

run: osdev.iso
	$(QEMU) -cdrom osdev.iso
