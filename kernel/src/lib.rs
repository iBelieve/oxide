#![feature(lang_items)]
#![no_std]

extern crate rlibc;

pub mod x86_64;

#[no_mangle]
pub extern fn kernel_main() {
    // TODO: Initialize the screen using the VGA module
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    // TODO: Use println macro from the VGA module
    // println!("\n\nPANIC in {} at line {}:", file, line);
    // println!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}
