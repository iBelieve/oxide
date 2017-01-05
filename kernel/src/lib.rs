#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
pub mod vga;

pub mod arch;

#[no_mangle]
pub extern fn kernel_main() {
    vga::init();

    println!("Kernel started.");

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
