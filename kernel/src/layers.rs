use core::cell::{OnceCell, RefCell};
use core::fmt::Write;

use spin::Mutex;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::error::{KernelError, KernelResult, LayerReason};
use kernel_lib::gop::console::DISPLAY_BACKGROUND_COLOR;
use kernel_lib::layers::console::console_colors::TextColors;
use kernel_lib::layers::console::TextLayer;
use kernel_lib::layers::count::CountLayer;
use kernel_lib::layers::cursor::CursorLayer;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::shape::shape_colors::ShapeColors;
use kernel_lib::layers::shape::shape_drawer::ShapeDrawer;
use kernel_lib::layers::shape::ShapeLayer;
use kernel_lib::layers::window::WindowLayer;
use kernel_lib::layers::{frame_buffer_layer_transform, Layers};

use crate::layers::window_keyboard::window_keyboard;

mod window_keyboard;

pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers(OnceCell<Mutex<RefCell<Layers>>>);


pub const BACKGROUND_LAYER_KEY: &str = "BACKGROUND";


pub const WINDOW_COUNT: &str = "WINDOW COUNT";
pub const COUNT: &str = "COUNT";


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
            .set(Mutex::new(RefCell::new(layers)))
            .map_err(|_| KernelError::FailedOperateLayer(LayerReason::FailedInitialize))
    }


    pub fn layers_mut(&self) -> &Mutex<RefCell<Layers>> {
        self.0.get().unwrap()
    }
}


unsafe impl Sync for GlobalLayers {}


pub fn init_layers(config: FrameBufferConfig) -> KernelResult {
    LAYERS.init(config)?;

    let biding = LAYERS.layers_mut();
    let layers = biding.lock();
    let mut layers = layers.borrow_mut();

    layers.new_layer(screen_background(config));
    layers.new_layer(console(config));
    layers.new_layer(time_count_window(config)?);
    layers.new_layer(window_keyboard(config)?);
    layers.new_layer(mouse(config));

    layers.draw_all_layer()
}


fn screen_background(config: FrameBufferConfig) -> LayerKey {
    let transform = frame_buffer_layer_transform(config);
    let colors = ShapeColors::default().change_foreground(DISPLAY_BACKGROUND_COLOR);
    let shape_drawer = ShapeDrawer::new(config, colors);

    ShapeLayer::new(shape_drawer, transform)
        .into_enum()
        .into_layer_key(BACKGROUND_LAYER_KEY)
}


fn time_count_window(config: FrameBufferConfig) -> KernelResult<LayerKey> {
    let transform = Transform2D::new(Vector2D::new(300, 100), Size::new(160, 52));

    let window = WindowLayer::new(config, transform.clone())
        .new_layer(count_layer(config, &transform)?)?
        .into_enum()
        .into_layer_key(WINDOW_COUNT);

    Ok(window)
}


fn count_layer(
    config: FrameBufferConfig,
    window_transform: &Transform2D,
) -> KernelResult<LayerKey> {
    let size = window_transform.size() - Size::new(20, 0);
    let pos = Vector2D::new(
        window_transform
            .size()
            .width()
            / 2
            - 32,
        0,
    );

    let count = CountLayer::new(
        config,
        Transform2D::new(pos, size.unwrap_or(window_transform.size())),
    )?;

    Ok(count
        .into_enum()
        .into_layer_key(COUNT))
}


fn console(config: FrameBufferConfig) -> LayerKey {
    TextLayer::new(
        config,
        Vector2D::zeros(),
        Size::new(50, 10),
        TextColors::default(),
    )
    .into_enum()
    .into_layer_key(CONSOLE_LAYER_KEY)
}


fn mouse(config: FrameBufferConfig) -> LayerKey {
    LayerKey::new(
        MOUSE_LAYER_KEY,
        CursorLayer::new_use_default(config).into_enum(),
    )
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    LAYERS
        .layers_mut()
        .lock()
        .borrow_mut()
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
