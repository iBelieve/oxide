#![feature(asm, const_fn, fixed_size_array, lang_items, unique, collections, alloc,
           box_syntax, drop_types_in_const, naked_functions, thread_local, core_intrinsics)]
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
extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate lazy_static;


pub use arch::kernel_start;
pub use runtime::*;

#[macro_use]
mod int_like;

#[macro_use]
mod arch;

mod bitmap;
mod time;
mod runtime;
mod tasking;


fn kernel_main() {
    tasking::init();

    println!("Hello, Rust kernel world!");

    tasking::spawn(separate_task);
    tasking::spawn(task_2);
    tasking::switch();
    println!("Back in main.");
}

fn separate_task() {
    println!("My new task!");
}

fn task_2() {
    println!("Second task!");
}
