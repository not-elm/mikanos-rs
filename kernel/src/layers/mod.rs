use core::cell::OnceCell;

use spin::Mutex;

use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::error::{KernelError, KernelResult, LayerReason};
use kernel_lib::gop::pixel::rc_pixel_writer;
use kernel_lib::layers::window::mouse_cursor_drawer::MouseCursorDrawer;
use kernel_lib::layers::window::status::builder::WindowBuilder;
use kernel_lib::layers::Layers;

pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers<'layer>(OnceCell<Mutex<Layers<'layer>>>);


impl<'layer> GlobalLayers<'layer> {
    pub const fn new_uninit() -> GlobalLayers<'layer> {
        Self(OnceCell::new())
    }

    pub fn init(&self, frame_buffer_config: FrameBufferConfig) -> KernelResult {
        self.0
            .set(Mutex::new(Layers::new_with_rc(rc_pixel_writer(
                frame_buffer_config,
            ))))
            .map_err(|_| KernelError::FailedOperateLayer(LayerReason::FailedInititialize))
    }


    pub fn mutex(&'layer self) -> &'layer Mutex<Layers<'layer>> {
        self.0.get().unwrap()
    }
}

unsafe impl<'layer> Sync for GlobalLayers<'layer> {}

pub fn init_layers(frame_buffer_config: FrameBufferConfig) -> KernelResult {
    LAYERS.init(frame_buffer_config)?;
    let mut layers = LAYERS.mutex().lock();
    let layer = layers.new_layer();
    let window = WindowBuilder::new().build(MouseCursorDrawer::default());

    layer.add_window("mouse", window);
    Ok(())
}
