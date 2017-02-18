#![no_std]
#![feature(lang_items)]


extern crate kernel;

pub extern "C" fn init() {
    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
}

// #[lang = "eh_personality"]
// extern fn eh_personality() {}
//
// #[lang = "panic_fmt"]
// #[no_mangle]
// pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
//     loop {}
// }
//
// #[allow(non_snake_case)]
// #[no_mangle]
// pub extern fn _Unwind_Resume() -> ! {
//     loop {}
// }
