use core::cell::OnceCell;
use core::fmt::Write;

use spin::{Mutex, MutexGuard};

use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::error::{KernelError, KernelResult, LayerReason};
use kernel_lib::layers::Layers;

use crate::layers::console::console;
use crate::layers::mouse::mouse;
use crate::layers::screen::screen_background;
use crate::layers::time_count::time_count_window;
use crate::layers::window_keyboard::window_keyboard;

mod console;
mod mouse;
mod screen;
mod time_count;
mod window_keyboard;

pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers(OnceCell<Mutex<Layers>>);


pub const BACKGROUND_LAYER_KEY: &str = "BACKGROUND";

pub const WINDOW_COUNT: &str = "WINDOW COUNT";
pub const COUNT_LAYER_KEY: &str = "COUNT";


pub const WINDOW_KEYBOARD: &str = "WINDOW KEYBOARD";
pub const KEYBOARD_TEXT: &str = "WINDOW TEXT";

pub const MOUSE_LAYER_KEY: &str = "MOUSE_CURSOR";

pub const CONSOLE_LAYER_KEY: &str = "CONSOLE";


impl GlobalLayers {
    pub const fn new_uninit() -> GlobalLayers {
        Self(OnceCell::new())
    }

    pub fn init(&self, frame_buffer_config: FrameBufferConfig) -> KernelResult {
        let layers = Layers::new(frame_buffer_config);

        self.0
            .set(Mutex::new(layers))
            .map_err(|_| KernelError::FailedOperateLayer(LayerReason::FailedInitialize))
    }


    pub fn lock(&self) -> MutexGuard<Layers> {
        self.0.get().unwrap().lock()
    }


    pub fn try_lock(&self) -> Option<MutexGuard<Layers>> {
        self.0.get().unwrap().try_lock()
    }
}


unsafe impl Sync for GlobalLayers {}


pub fn init_layers(config: FrameBufferConfig) -> KernelResult {
    LAYERS.init(config)?;

    let mut layers = LAYERS.lock();

    layers.new_layer(screen_background(config));
    layers.new_layer(console(config));
    layers.new_layer(time_count_window(config)?);
    layers.new_layer(window_keyboard(config)?);
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
