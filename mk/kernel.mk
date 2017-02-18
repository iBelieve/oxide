SRC_DIR    = kernel/src
ARCH_DIR   = $(SRC_DIR)/arch/$(ARCH)

# Source & output files
ASM_SRC_FILES := $(wildcard $(ARCH_DIR)/*.asm)
ASM_OUT_FILES := $(patsubst $(ARCH_DIR)/%.asm, $(OUT_DIR)/%.o, $(ASM_SRC_FILES))
SRC_FILES     := $(shell find $(SRC_DIR) -name '*.rs')

$(TARGET_DIR)/libkernel.a: Cargo.toml kernel/Cargo.toml $(SRC_FILES)
	$(CARGO) build --target=$(TARGET) --package kernel

$(KERNEL): $(ARCH_DIR)/linker.ld $(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(LD) --gc-sections --nmagic -T $(ARCH_DIR)/linker.ld -o $(KERNEL) \
			$(ASM_OUT_FILES) $(TARGET_DIR)/libkernel.a
	$(GRUB_FILE) --is-x86-multiboot2 $(KERNEL)

$(OUT_DIR)/%.o: $(ARCH_DIR)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
