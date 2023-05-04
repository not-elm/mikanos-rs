use alloc::rc::Rc;
use core::cell::{OnceCell, RefCell, RefMut};
use core::fmt::Write;

use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::error::{KernelError, KernelResult, LayerReason};
use kernel_lib::gop::console::DISPLAY_BACKGROUND_COLOR;
use kernel_lib::layers::console::console_colors::ConsoleColors;
use kernel_lib::layers::console::ConsoleLayer;
use kernel_lib::layers::cursor::CursorLayer;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::shape::shape_colors::ShapeColors;
use kernel_lib::layers::shape::shape_drawer::ShapeDrawer;
use kernel_lib::layers::shape::ShapeLayer;
use kernel_lib::layers::{frame_buffer_layer_transform, Layers};

pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers(OnceCell<Rc<RefCell<Layers>>>);


struct KernelLayers(Layers);


pub const BACKGROUND_LAYER_KEY: &str = "BACKGROUND";


pub const MOUSE_LAYER_KEY: &str = "MOUSE_CURSOR";


pub const CONSOLE_LAYER_KEY: &str = "CONSOLE";


impl GlobalLayers {
    pub const fn new_uninit() -> GlobalLayers {
        Self(OnceCell::new())
    }

    pub fn init(&self, frame_buffer_config: FrameBufferConfig) -> KernelResult {
        let layers = Layers::new(frame_buffer_config);

        self.0
            .set(Rc::new(RefCell::new(layers)))
            .map_err(|_| KernelError::FailedOperateLayer(LayerReason::FailedInitialize))
    }


    pub fn layers_mut(&self) -> Rc<RefCell<Layers>> {
        Rc::clone(self.0.get().unwrap())
    }
}


unsafe impl Sync for GlobalLayers {}


pub fn init_layers(frame_buffer_config: FrameBufferConfig) -> KernelResult {
    LAYERS.init(frame_buffer_config)?;

    let layers = LAYERS.layers_mut();
    let mut layers = layers.borrow_mut();

    add_background_layer(frame_buffer_config, &mut layers);
    add_console_layer(frame_buffer_config, &mut layers);
    add_mouse_layer(frame_buffer_config, &mut layers);

    layers.draw_all_layer()
}


fn add_background_layer(frame_buffer_config: FrameBufferConfig, layers: &mut RefMut<Layers>) {
    let transform = frame_buffer_layer_transform(frame_buffer_config);
    let shape_drawer = ShapeDrawer::new(
        frame_buffer_config,
        ShapeColors::default().change_foreground(DISPLAY_BACKGROUND_COLOR),
    );


    layers.new_layer(
        ShapeLayer::new(shape_drawer, transform)
            .into_enum()
            .into_layer_key(BACKGROUND_LAYER_KEY),
    );
}


fn add_console_layer(config: FrameBufferConfig, layers: &mut RefMut<Layers>) {
    layers.new_layer(
        ConsoleLayer::new(config, ConsoleColors::default())
            .into_enum()
            .into_layer_key(CONSOLE_LAYER_KEY),
    );
}


fn add_mouse_layer(config: FrameBufferConfig, layers: &mut RefMut<Layers>) {
    let cursor_layer = LayerKey::new(
        MOUSE_LAYER_KEY,
        CursorLayer::new_use_default(config).into_enum(),
    );


    layers.new_layer(cursor_layer);
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    LAYERS
        .layers_mut()
        .borrow_mut()
        .update_layer(CONSOLE_LAYER_KEY, |layer| {
            layer
                .require_console()
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
