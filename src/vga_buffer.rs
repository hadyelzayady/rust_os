//disable warning of unused variant in color enum
#[allow(dead_code)]
//to instruct the compiler to provide basic implementations for these traits (for complex behavior these traits should be implemented manually)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//represent each enum variant as unsigned 8-bit integer
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
//# these called attribute.
//derive attribute tell  rust to automatically implements these traits (debug, clone,..) for the following struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//To make memory layout of ColorCode exactly like its signle field u8 memory layout
#[repr(transparent)]
//tuple struct but with one field
struct ColorCode(u8); //contains u8 field

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//the field ordering is undefined by default so here we set the ordering to be like C-lang
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//make memory layout of Buffer exactly like its single field
//as the pointer we use points to buffer and must also point to chars so if we say Buffer[i][j] this equal Buffer.chars[i][j]
//so Buffer must have the memory layout of its member chars

//The problem is that we only write to the Buffer and never read from it again. The compiler doesn't know that we really access VGA buffer memory (instead of normal RAM) and knows nothing about the side effect that some characters appear on the screen. So it might decide that these writes are unnecessary and can be omitted. To avoid this erroneous optimization, we need to specify these writes as volatile. This tells the compiler that the write has side effects and should not be optimized away.
use volatile::Volatile;
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    // 'static lifetime specifies that the refenence is valid for the whole program
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                //after using volatile:Instead of a normal assignment using =, we're now using the write method. This guarantees that the compiler will never optimize away this write.
                self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //printable ASCII byte or newline , as str is utf-8 which means some characters needs two bytes but in vga only one byte is available for a char
                // | separates between multiple patterns to match
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.row_position += 1;
        if self.row_position >= BUFFER_HEIGHT {
            self.shift_up();
            self.row_position = BUFFER_HEIGHT - 1;
        }
        self.column_position = 0;
    }

    /// Shift the VGA content one line up
    ///
    ///
    fn shift_up(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
            }
        }
        //clear last line
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_HEIGHT {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

use core::fmt;
//to use formatting macros we should implement Write trait which only contains method write_str
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("A\nB\nC\nD\nE\nF\nG\nG\nW\nX\nY\nZ\n1\n2\n3\n4\n5\n6\n7\n8\n\n\n\n\n");

    //The write! call returns a Result which causes a warning if not used, so we call the unwrap function on it, which panics if an error occurs
    //after implement fmt::write trait we can use format string now using write macro
    use core::fmt::Write;
    //write return Result but we do  not use it so to remove compiler warning of unused return, we call unwrap which panics if error happened
    write!(writer, "the numbers are {} and {}\n\n", 42, 10 / 3).unwrap();
}
