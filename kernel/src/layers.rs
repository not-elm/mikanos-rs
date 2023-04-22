use alloc::rc::Rc;
use core::cell::{OnceCell, RefCell};

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use kernel_lib::error::{KernelError, KernelResult, LayerReason};
use kernel_lib::gop::pixel::rc_pixel_writer;
use kernel_lib::layers::layer_status::LayerStatusBuilder;
use kernel_lib::layers::window::drawers::mouse_cursor::MouseCursorDrawer;
use kernel_lib::layers::window::status::builder::WindowStatusBuilder;
use kernel_lib::layers::window::Window;
use kernel_lib::layers::Layers;

pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers<'layer>(OnceCell<Rc<RefCell<Layers<'layer>>>>);


impl<'layer> GlobalLayers<'layer> {
    pub const fn new_uninit() -> GlobalLayers<'layer> {
        Self(OnceCell::new())
    }

    pub fn init(&self, frame_buffer_config: FrameBufferConfig) -> KernelResult {
        let layers = Layers::new_with_rc(rc_pixel_writer(frame_buffer_config));

        self.0
            .set(Rc::new(RefCell::new(layers)))
            .map_err(|_| KernelError::FailedOperateLayer(LayerReason::FailedInitialize))
    }


    pub fn layers_mut(&'static self) -> Rc<RefCell<Layers>> {
        Rc::clone(self.0.get().unwrap())
    }
}

unsafe impl<'layer> Sync for GlobalLayers<'layer> {}

pub fn init_layers(frame_buffer_config: FrameBufferConfig) -> KernelResult {
    LAYERS.init(frame_buffer_config)?;

    let layers = LAYERS.layers_mut();
    let mut layers = layers.borrow_mut();

    let layer = layers.new_layer(
        LayerStatusBuilder::new(Size::new(
            frame_buffer_config.horizontal_resolution,
            frame_buffer_config.vertical_resolution,
        ))
        .build(),
    );

    let window_status = WindowStatusBuilder::new().build();
    let window = Window::new(MouseCursorDrawer::default(), window_status);

    layer.add_window("mouse", window);
    Ok(())
}
