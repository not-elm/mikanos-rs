use core::fmt::Write;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop;
use kernel_lib::layers::LAYERS;

use crate::layers::console::console;
use crate::layers::desktop::desktop;
use crate::layers::mouse::mouse;
use crate::layers::time_count::time_count_window;
use crate::layers::window_keyboard::window_keyboard;

mod console;
mod mouse;
mod desktop;
mod time_count;
mod window_keyboard;


pub const DESKTOP_LAYER_KEY: &str = "BACKGROUND";
pub const COUNT_TEXT_LAYER_KEY: &str = "COUNT";
pub const COUNT_TEXT_LAYER2_KEY: &str = "COUNT 2";
pub const WINDOW_KEYBOARD: &str = "WINDOW KEYBOARD";
pub const KEYBOARD_TEXT: &str = "WINDOW TEXT";
pub const MOUSE_LAYER_KEY: &str = "MOUSE_CURSOR";
pub const CONSOLE_LAYER_KEY: &str = "CONSOLE";


pub fn init_layers(config: FrameBufferConfig) -> KernelResult {
    gop::init(config);

    LAYERS.init(config)?;

    let mut layers = LAYERS.lock();

    layers.new_layer(desktop());
    layers.new_layer(console(config));
    layers.new_layer(time_count_window(config, "Count1", Vector2D::new(100, 100), COUNT_TEXT_LAYER_KEY, "Count Window 1")?);
    layers.new_layer(time_count_window(config, "Count2", Vector2D::new(100, 200), COUNT_TEXT_LAYER2_KEY, "Count Window 2")?);
    layers.new_layer(window_keyboard()?);

    layers.new_layer(mouse(config));

    layers.draw_all_layer()
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    LAYERS
        .lock()
        .update_layer(CONSOLE_LAYER_KEY, |layer| {
            layer
                .require_text()
                .unwrap()
                .write_fmt(args)
                .unwrap();
        })
        .unwrap();
}


#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ($crate::layers::_print(format_args!($($args)*)));
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
