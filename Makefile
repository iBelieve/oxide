ARCH=x86_64
TARGET=$(ARCH)-unknown-none-gnu

# Directories
PREFIX=tools/bin
TARGET_DIR=target/$(TARGET)/debug
OUT_DIR=$(TARGET_DIR)/build
SRC_DIR=kernel/src
ARCH_DIR=$(SRC_DIR)/arch/$(ARCH)

# Source & output files
ASM_SRC_FILES := $(wildcard $(ARCH_DIR)/*.asm)
ASM_OUT_FILES := $(patsubst $(ARCH_DIR)/%.asm, $(OUT_DIR)/%.o, $(ASM_SRC_FILES))
SRC_FILES := $(shell find $(SRC_DIR) -name '*.rs')

# Tools
MAKE_ISO=tools/bin/grub-mkrescue
QEMU=qemu-system-x86_64
CARGO=xargo
LD=$(PREFIX)/x86_64-elf-ld

.PHONY: all run clean

all: osdev.iso

iso: osdev.iso

$(TARGET_DIR)/libkernel.a: Cargo.toml kernel/Cargo.toml $(SRC_FILES)
	$(CARGO) build --target=$(TARGET) --package kernel

$(TARGET_DIR)/osdev.bin: $(ARCH_DIR)/linker.ld $(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(LD) --gc-sections --nmagic -T $(ARCH_DIR)/linker.ld -o $(TARGET_DIR)/osdev.bin \
			$(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a

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
