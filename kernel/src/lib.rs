#![feature(asm, const_fn, fixed_size_array, lang_items, unique, alloc, box_syntax,
           drop_types_in_const, naked_functions, thread_local, core_intrinsics,
           const_max_value, const_atomic_usize_new, const_unique_new)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
#[macro_use]
extern crate once;
extern crate rlibc;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate x86;
#[macro_use]
extern crate alloc_kernel;
#[macro_use]
extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate tar;

pub use arch::kernel_start;
pub use runtime::*;

use core::str::from_utf8;

#[macro_use]
mod int_like;

#[macro_use]
mod logging;

#[macro_use]
mod arch;

mod bitmap;
mod time;
mod runtime;
mod tasking;
mod filesystem;


fn kernel_main() {
    tasking::init();

    println!("Hello, Rust kernel world!");

    tasking::spawn(separate_task);
    tasking::spawn(task_2);
    tasking::switch();
    println!("Back in main.");

    if let Some(mut file) = filesystem::fs().get_file("/initrd/hello.txt") {
        ok!("Found file: {}", from_utf8(file.read().as_slice()).expect("Unable to decode file"));
    } else {
        fail!("File not found :(");
    }
}

fn separate_task() {
    println!("My new task!");
}

fn task_2() {
    println!("Second task!");
}
