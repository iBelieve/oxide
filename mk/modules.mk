$(TARGET_DIR)/harddisk.ko: $(TARGET_DIR)/libkernel.a Cargo.toml modules/harddisk/Cargo.toml modules/harddisk/src/**
	$(CARGO) build --target=$(TARGET) --package harddisk
	$(LD) -r --require-defined init -o $(TARGET_DIR)/harddisk.ko $(TARGET_DIR)/libharddisk.rlib
