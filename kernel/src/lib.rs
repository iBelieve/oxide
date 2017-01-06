#![feature(asm)]
#![feature(const_fn)]
#![feature(fixed_size_array)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate multiboot2;

#[macro_use]
pub mod vga;

pub mod arch;
pub mod bitmap;

use arch::mem::VirtualAddress;

extern {
    static kernel_end: usize;
}

fn get_kernel_end() -> VirtualAddress {
    unsafe { (&kernel_end as *const usize) as VirtualAddress }
}

#[no_mangle]
pub extern fn kernel_main(multiboot_address: usize) {
    let boot_info = unsafe{ multiboot2::load(multiboot_address) };

    vga::init();

    println!("Kernel started.");

    arch::mem::init(boot_info, get_kernel_end());

    // TODO: Other initialization code here

    println!("Hello, Rust kernel world!");
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}
