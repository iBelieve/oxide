#![feature(asm, const_fn, fixed_size_array, lang_items, unique, collections)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
#[macro_use]
extern crate once;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86;
#[macro_use]
extern crate alloc_kernel;
#[macro_use]
extern crate collections;


pub use arch::kernel_start;
pub use runtime::*;

#[macro_use]
mod arch;

mod bitmap;
mod time;
mod runtime;


pub extern fn kernel_main() {
    println!("Hello, Rust kernel world!");
}
