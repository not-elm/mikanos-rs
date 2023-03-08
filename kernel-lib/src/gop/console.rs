use core::fmt::Write;

use crate::error::KernelResult;
use crate::gop::console::console_builder::ConsoleBuilder;
use crate::gop::console::console_writer::ConsoleWriter;
use crate::gop::pixel::pixel_color::PixelColor;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;
use spin::Mutex;

pub mod console_builder;
pub mod console_writer;

pub struct GlobalConsole(Option<Mutex<ConsoleWriter>>);
pub static mut CONSOLE: GlobalConsole = GlobalConsole(None);

const CURSOR_WIDTH: usize = 15;
const CURSOR_HEIGHT: usize = 24;
const CURSOR_SHAPE: [&'static [u8; CURSOR_WIDTH]; CURSOR_HEIGHT] = [
    b"@              ",
    b"@@             ",
    b"@.@            ",
    b"@..@           ",
    b"@...@          ",
    b"@....@         ",
    b"@.....@        ",
    b"@......@       ",
    b"@.......@      ",
    b"@........@     ",
    b"@.........@    ",
    b"@..........@   ",
    b"@...........@  ",
    b"@............@ ",
    b"@......@@@@@@@@",
    b"@......@       ",
    b"@....@@.@      ",
    b"@...@ @.@      ",
    b"@..@   @.@     ",
    b"@.@    @.@     ",
    b"@@      @.@    ",
    b"@       @.@    ",
    b"         @.@   ",
    b"         @@@   ",
];

impl GlobalConsole {
    pub fn init(&mut self, frame_buffer_config: FrameBufferConfig) {
        let console = ConsoleBuilder::new().build(frame_buffer_config);
        self.0 = Some(Mutex::new(console));
    }

    pub fn get_mut(&mut self) -> &mut ConsoleWriter {
        self.0.as_mut().unwrap().get_mut()
    }
}

pub fn init_console(frame_buffer_config: FrameBufferConfig) {
    unsafe { CONSOLE.init(frame_buffer_config) };
}

pub fn draw_cursor() -> KernelResult {
    for dy in 0..CURSOR_HEIGHT {
        for dx in 0..CURSOR_WIDTH {
            let c = char::from(CURSOR_SHAPE[dy][dx]);
            if c == '@' {
                get_mut_console()
                    .write_pixel(Vector2D::new(200 + dx, 100 + dy), PixelColor::new(0, 0, 0))?;
            } else if c == '.' {
                get_mut_console().write_pixel(
                    Vector2D::new(200 + dx, 100 + dy),
                    PixelColor::new(0xFF, 0xFF, 0xFF),
                )?;
            }
        }
    }

    Ok(())
}
pub fn get_mut_console() -> &'static mut ConsoleWriter {
    unsafe { CONSOLE.get_mut() }
}

#[doc(hidden)]
pub fn _print(s: core::fmt::Arguments) {
    get_mut_console().write_fmt(s).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ($crate::gop::console::_print(format_args!($($args)*)));
}

#[macro_export]
macro_rules! println {
        () => {
            $crate::print!("\n");
        };
        ($fmt: expr) => {
           $crate::print!(concat!($fmt, "\n"));
       };
       ($fmt: expr, $($args:tt)*) => {
           $crate::print!(concat!($fmt,"\n"), $($args)*);
       };
}
