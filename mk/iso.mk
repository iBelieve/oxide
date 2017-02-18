$(ISO): $(KERNEL) $(TARGET_DIR)/harddisk.ko data/grub.cfg data/initrd/*
	mkdir -p $(TARGET_DIR)/isodir/boot/grub
	cp $(KERNEL) $(TARGET_DIR)/isodir/boot
	cp data/grub.cfg $(TARGET_DIR)/isodir/boot/grub

	rm -r $(TARGET_DIR)/initrd || true
	cp -r data/initrd $(TARGET_DIR)/initrd
	cp $(TARGET_DIR)/harddisk.ko $(TARGET_DIR)/initrd
	tar -cf $(TARGET_DIR)/isodir/boot/$(NAME).initrd -C $(TARGET_DIR)/initrd .

	$(MAKE_ISO) -o $(ISO) $(TARGET_DIR)/isodir
	@test -f $(ISO) || { echo "ISO not created correctly!"; exit 1; }
