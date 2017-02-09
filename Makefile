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
GRUB_FILE=tools/bin/grub-file
QEMU=qemu-system-x86_64
CARGO=xargo
LD=$(PREFIX)/x86_64-elf-ld
QEMU_ARGS=-m size=256

.PHONY: all run clean

all: oxide.iso

iso: oxide.iso

test: oxide.iso

$(TARGET_DIR)/libkernel.a: Cargo.toml kernel/Cargo.toml $(SRC_FILES)
	$(CARGO) build --target=$(TARGET) --package kernel

$(TARGET_DIR)/oxide.bin: $(ARCH_DIR)/linker.ld $(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(LD) --gc-sections --nmagic -T $(ARCH_DIR)/linker.ld -o $(TARGET_DIR)/oxide.bin \
			$(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(GRUB_FILE) --is-x86-multiboot2 $(TARGET_DIR)/oxide.bin

$(OUT_DIR)/%.o: $(ARCH_DIR)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

oxide.iso: $(TARGET_DIR)/oxide.bin data/grub.cfg data/initrd/*
	mkdir -p $(TARGET_DIR)/isodir/boot/grub
	cp $(TARGET_DIR)/oxide.bin $(TARGET_DIR)/isodir/boot
	cp data/grub.cfg $(TARGET_DIR)/isodir/boot/grub
	rm -r $(TARGET_DIR)/initrd || true
	cp -r data/initrd $(TARGET_DIR)/initrd
	tar -cf $(TARGET_DIR)/isodir/boot/oxide.initrd -C $(TARGET_DIR)/initrd .
	$(MAKE_ISO) -o oxide.iso $(TARGET_DIR)/isodir
	@test -f oxide.iso || { echo "ISO not created correctly!"; exit 1; }

run: oxide.iso
	$(QEMU) -cdrom oxide.iso $(QEMU_ARGS) -s

debug: oxide.iso
	$(QEMU) -cdrom oxide.iso $(QEMU_ARGS) -s -S

debug-exception: oxide.iso
	$(QEMU) -cdrom oxide.iso $(QEMU_ARGS) -d int -no-reboot
