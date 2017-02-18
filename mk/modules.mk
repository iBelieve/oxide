$(TARGET_DIR)/harddisk.ko: Cargo.toml modules/harddisk/Cargo.toml modules/harddisk/src/**
	$(CARGO) build --target=$(TARGET) --package harddisk
	mv $(TARGET_DIR)/libharddisk.rlib $(TARGET_DIR)/harddisk.ko
