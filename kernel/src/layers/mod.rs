use core::cell::OnceCell;

use spin::{Mutex, MutexGuard};

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;
use kernel_lib::gop::pixel::rc_pixel_writer;
use kernel_lib::gop::pixel::rc_pixel_writer::RcPixelWriter;
use kernel_lib::layers::layer::Layer;
use kernel_lib::layers::window::builder::WindowBuilder;
use kernel_lib::layers::window::mouse_cursor_window::MouseCursorDrawer;
use kernel_lib::layers::Layers;

pub static mut LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers(OnceCell<Mutex<Layers<'static>>>);

impl GlobalLayers {
    pub const fn new_uninit() -> Self {
        Self(OnceCell::new())
    }

    pub fn init(&self, frame_buffer_config: FrameBufferConfig) {
        self.0
            .set(Mutex::new(Layers::new_with_rc(rc_pixel_writer(
                frame_buffer_config,
            ))));
    }

    pub fn lock(&'static self) -> MutexGuard<'static, Layers<'static>> {
        self.0.get().unwrap().lock()
    }

    pub fn layer_at(
        &'static mut self,
        id: usize,
    ) -> Option<&'static mut Layer<'static, RcPixelWriter<'static>>> {
        self.0
            .get_mut()
            .unwrap()
            .get_mut()
            .layer_mut_at(id)
    }


    pub fn get_mut(&'static mut self) -> &'static mut Layers<'static> {
        self.0
            .get_mut()
            .unwrap()
            .get_mut()
    }
}

unsafe impl Sync for GlobalLayers {}

pub fn init_layers(frame_buffer_config: FrameBufferConfig) {
    unsafe {
        LAYERS.init(frame_buffer_config);
        let mut layers = LAYERS.lock();
        let layer = layers.new_layer();
        let window = WindowBuilder::new().build(MouseCursorDrawer::default());

        layer.add_window("mouse", window);
    }
}
