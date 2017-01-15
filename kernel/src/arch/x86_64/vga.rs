use arch::io::PortPair;
use arch::mem::VGA_BUFFER;
use core::fmt;
use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

/***** MACROS *****/

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::arch::vga::print(format_args!($($arg)*));
    });
}

/***** CONSTANTS *****/

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

/***** GLOBAL VARIABLES *****/

static VGA: Mutex<PortPair<u8>> = Mutex::new(unsafe { PortPair::new(0x3D4, 0x3D5) });

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    row: 0,
    column: 0,
    color_code: ColorCode::new(Color::LightGray, Color::Black),
    buffer: unsafe { Unique::new(VGA_BUFFER as *mut _) },
});

/***** ENUMS AND STRUCTS *****/

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row: usize,
    column: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row;
                let col = self.column;
                let color_code = self.color_code;

                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });

                self.column += 1;
            }
        }

        move_cursor(self.row, self.column);
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        self.column = 0;
        self.row += 1;

        if self.row >= BUFFER_HEIGHT {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        // Move all but the first row up one row
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][column].read();
                buffer.chars[row - 1][column].write(character);
            }
        }

        // Clear the last row
        let last_row = BUFFER_HEIGHT - 1;

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for column in 0..BUFFER_WIDTH {
            self.buffer().chars[last_row][column].write(blank);
        }

        self.row -= 1;
    }

    pub fn clear(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for row in 0..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                self.buffer().chars[row][column].write(blank);
            }
        }

        self.row = 0;
        self.column = 0;

        move_cursor(self.row, self.column);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

/***** FUNCTIONS *****/

pub fn init() {
    assert_has_not_been_called!("vga::init must be called only once");

    WRITER.lock().clear();
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

fn move_cursor(row: usize, column: usize) {
    let cursor_index = row * BUFFER_WIDTH + column;

    /* This sends a command to indicies 14 and 15 in the
    *  CRT Control Register of the VGA controller. These
    *  are the high and low bytes of the index that show
    *  where the hardware cursor is to be 'blinking'. To
    *  learn more, you should look up some VGA specific
    *  programming documents. A great start to graphics:
    *  http://www.brackeen.com/home/vga */
    VGA.lock().write(14, (cursor_index >> 8) as u8);
    VGA.lock().write(15, cursor_index as u8);
}
