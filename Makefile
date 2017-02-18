NAME   = oxide
ARCH   = x86_64
TARGET = $(ARCH)-$(NAME)

KERNEL 	   = $(TARGET_DIR)/$(NAME).bin
ISO    = $(NAME).iso

PREFIX     = tools/bin
TARGET_DIR = target/$(TARGET)/debug
OUT_DIR    = $(TARGET_DIR)/build

# Tools
CARGO	  = xargo
MAKE_ISO  = tools/bin/grub-mkrescue
GRUB_FILE = tools/bin/grub-file
QEMU      = qemu-system-$(ARCH)
LD        = $(PREFIX)/$(ARCH)-elf-ld
QEMU_ARGS = -curses -m size=256

.PHONY: all run debug tools clean

all: iso

tools:
	./build_tools.sh

clean:
	rm -r target
	rm $(ISO)

iso: $(ISO)

run: $(ISO)
	$(QEMU) -cdrom $(ISO) $(QEMU_ARGS) -s

debug: $(ISO)
	$(QEMU) -cdrom $(ISO) $(QEMU_ARGS) -s -S

debug-exception: $(ISO)
	$(QEMU) -cdrom $(ISO) $(QEMU_ARGS) -d int -no-reboot

$(OUT_DIR)/%.o: $(ARCH_DIR)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

run: $(ISO)
	$(QEMU) -cdrom $(ISO) $(QEMU_ARGS) -s

debug: $(ISO)
	$(QEMU) -cdrom $(ISO) $(QEMU_ARGS) -s -S

debug-exception: $(ISO)
	$(QEMU) -cdrom $(ISO) $(QEMU_ARGS) -d int -no-reboot

include mk/kernel.mk
include mk/modules.mk
include mk/iso.mk
