use arch::mem::VirtualAddress;
use arch::*;
use multiboot2;
use ::kernel_main;

extern {
    static mut __end: u8;
}

#[no_mangle]
pub extern fn kernel_start(multiboot_address: usize) {
    let boot_info = unsafe { multiboot2::load(multiboot_address) };
    let kernel_end = unsafe { &__end as *const u8 as VirtualAddress };

    vga::init();

    println!("Kernel started.");

    clock::init();
    mem::init(boot_info, kernel_end);

    // TODO: Other initialization code here

    kernel_main();
}
