use arch::vga::{print, print_colored, Color};

macro_rules! log {
    ($status:expr, $color:expr, $fmt:expr) => {
        $crate::logging::status($status, $color);
        println!($fmt);
    };
    ($status:expr, $color:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::logging::status($status, $color);
        println!($fmt, $($arg)*);
    };
}

macro_rules! ok {
    ($fmt:expr) => (log!("OK", $crate::arch::vga::Color::Green, $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!("OK", $crate::arch::vga::Color::Green, $fmt, $($arg)*));
}

macro_rules! info {
    ($fmt:expr) => (log!("INFO", $crate::arch::vga::Color::Blue, $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!("INFO", $crate::arch::vga::Color::Blue, $fmt, $($arg)*));
}

pub fn status(label: &str, color: Color) {
    print(format_args!("[ "));
    print_colored(format_args!("{}", label), color);
    print(format_args!(" ] "));
}
