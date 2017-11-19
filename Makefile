NAME   = oxide
ARCH   = x86_64
TARGET = $(ARCH)-$(NAME)

KERNEL = $(TARGET_DIR)/$(NAME).bin
ISO    = $(NAME).iso

# Directories
PREFIX     = tools/bin
SRC_DIR    = kernel/src
ARCH_DIR   = $(SRC_DIR)/arch/$(ARCH)
TARGET_DIR = target/$(TARGET)/debug
OUT_DIR    = $(TARGET_DIR)/build

# Source & output files
ASM_SRC_FILES := $(wildcard $(ARCH_DIR)/*.asm)
ASM_OUT_FILES := $(patsubst $(ARCH_DIR)/%.asm, $(OUT_DIR)/%.o, $(ASM_SRC_FILES))
SRC_FILES     := $(shell find $(SRC_DIR) -name '*.rs')

# Tools
MAKE_ISO  = tools/bin/grub-mkrescue
GRUB_FILE = tools/bin/grub-file
QEMU      = qemu-system-$(ARCH)
LD        = $(PREFIX)/$(ARCH)-elf-ld
QEMU_ARGS = -m size=256

.PHONY: all run debug tools clean

all: iso

tools:
	./build_tools.sh

iso: $(ISO)

run: $(ISO)
	$(QEMU) -cdrom $< $(QEMU_ARGS) -s

debug: $(ISO)
	$(QEMU) -cdrom $< $(QEMU_ARGS) -s -S

debug-exception: $(ISO)
	$(QEMU) -cdrom $< $(QEMU_ARGS) -d int -no-reboot

$(OUT_DIR)/%.o: $(ARCH_DIR)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

$(TARGET_DIR)/libkernel.a: Cargo.toml kernel/Cargo.toml $(SRC_FILES)
	xargo build --target=$(TARGET) --package kernel

$(KERNEL): $(ARCH_DIR)/linker.ld $(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(LD) --gc-sections --nmagic -T $(ARCH_DIR)/linker.ld -o $(TARGET_DIR)/oxide.bin \
			$(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(GRUB_FILE) --is-x86-multiboot2 $(TARGET_DIR)/oxide.bin

$(ISO): $(KERNEL) data/grub.cfg data/initrd/*
	mkdir -p $(TARGET_DIR)/isodir/boot/grub
	cp $(KERNEL) $(TARGET_DIR)/isodir/boot
	cp data/grub.cfg $(TARGET_DIR)/isodir/boot/grub
	rm -r $(TARGET_DIR)/initrd || true
	cp -r data/initrd $(TARGET_DIR)/initrd
	tar -cf $(TARGET_DIR)/isodir/boot/$(NAME).initrd -C $(TARGET_DIR)/initrd .
	$(MAKE_ISO) -o $(ISO) $(TARGET_DIR)/isodir
	@test -f $(iSO) || { echo "ISO not created correctly!"; exit 1; }
