//https://os.phil-opp.com/vga-text-mode/
#[allow(dead_code)] //Normally the compiler would issue a warning for each unused variant. By using the #[allow(dead_code)] attribute, we disable these warnings for the Color enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //By deriving the Copy, Clone, Debug, PartialEq, and Eq traits, we enable copy semantics for the type and make it printable and comparable.
#[repr(u8)] //Because of the repr(u8) attribute, each enum variant is stored as a u8. Actually 4 bits would be sufficient, but Rust doesn’t have a u4 type.
pub enum Color { //We use a C-like enum here to explicitly specify the number for each color.
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
#[repr(transparent)] //To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute
struct ColorCode(u8); //To represent a full color code that specifies foreground and background color, we create a newtype on top of u8

impl ColorCode { //this is the ColorCode implementation
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//TEXT BUFFER

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] //we need the repr(C) attribute. It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] //For the Buffer struct, we use repr(transparent) again to ensure that it has the same memory layout as its single field.
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//To actually write to screen, we now create a writer type
//The writer will always write to the last line and shift lines up when a line is full (or on \n)
//
pub struct Writer {
    column_position: usize, // The column_position field keeps track of the current position in the last row.
    color_code: ColorCode, //The current foreground and background colors are specified by color_code
    buffer: &'static mut Buffer, //a reference to the VGA buffer is stored in buffer
}
