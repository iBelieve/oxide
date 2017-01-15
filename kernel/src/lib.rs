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

use arch::mem::VirtualAddress;
use core::ops::DerefMut;

extern {
    static kernel_end: usize;
}

fn get_kernel_end() -> VirtualAddress {
    unsafe { (&kernel_end as *const usize) as VirtualAddress }
}

#[no_mangle]
pub extern fn kernel_main(multiboot_address: usize) {
    let boot_info = unsafe{ multiboot2::load(multiboot_address) };

    arch::vga::init();

    println!("Kernel started.");

    arch::clock::init();
    arch::mem::init(boot_info, get_kernel_end());

    // TODO: Other initialization code here

    println!("Hello, Rust kernel world!");

    arch::mem::test_paging(arch::mem::pmm::ALLOCATOR.lock().deref_mut());
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
