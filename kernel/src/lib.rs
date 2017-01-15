#![feature(asm)]
#![feature(const_fn)]
#![feature(fixed_size_array)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
pub mod arch;

pub mod bitmap;
pub mod time;

pub extern fn kernel_main() {
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
