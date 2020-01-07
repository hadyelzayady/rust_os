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
//so Buffer must have the memory layout of its member chars (it is like Buffer is an alias)
#[repr(transparent)]
struct Buffer {
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	// 'static lifetime specifies that the refenence is valid for the whole program
	buffer: &'static mut Buffer,
}
impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if (self.column_position >= BUFFER_WIDTH) {
					self.new_line();
				}

				self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position] = ScreenChar {
					ascii_character: byte,
					color_code: self.color_code,
				};

				self.column_position += 1;
			}
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				//printable ASCII byte or newline
				// | separates between multiple patterns to match
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}

	fn new_line(&mut self) {}
}

pub fn print_something() {
	let mut writer = Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};

	writer.write_string("Hello world");
}
