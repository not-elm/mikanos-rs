use core::cell::OnceCell;
use core::fmt::{Error, Write};

use spin::Mutex;

use common_lib::frame_buffer::FrameBufferConfig;

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::console::console_writer::ConsoleWriter;
use crate::gop::pixel::pixel_color::PixelColor;

pub mod console_builder;
pub mod console_writer;


struct GlobalConsole {
    config: FrameBufferConfig,
    writer: ConsoleWriter<AscIICharWriter>,
}

pub struct GlobalMutexConsole(OnceCell<Mutex<GlobalConsole>>);

impl GlobalMutexConsole {
    pub const fn uninit() -> Self {
        Self(OnceCell::new())
    }

    pub fn init(&self, config: FrameBufferConfig) {
        let console = GlobalConsole {
            config,
            writer: ConsoleWriter::new(config, AscIICharWriter::default(), PixelColor::yellow()),
        };
        self.0
            .set(Mutex::new(console));
    }


    pub fn print(&mut self, s: &str) -> KernelResult {
        let mut console = unsafe {
            self.0
                .get_mut()
                .unwrap()
                .lock()
        };

        let frame_buff = unsafe {
            core::slice::from_raw_parts_mut(
                console
                    .config
                    .frame_buffer_base_ptr(),
                console
                    .config
                    .frame_buffer_size,
            )
        };
        console
            .writer
            .write_str(frame_buff, s)
    }
}


unsafe impl Sync for GlobalMutexConsole {}

pub static mut CONSOLE: GlobalMutexConsole = GlobalMutexConsole::uninit();

pub const DISPLAY_BACKGROUND_COLOR: PixelColor = PixelColor::new(0x33, 0x33, 0x33);


pub fn init_console(frame_buffer_config: FrameBufferConfig) {
    unsafe { CONSOLE.init(frame_buffer_config) };
}


impl core::fmt::Write for GlobalMutexConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.print(s) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::default()),
        }
    }
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    unsafe {
        CONSOLE
            .write_fmt(args)
            .unwrap();
    }
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
