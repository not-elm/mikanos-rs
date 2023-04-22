use alloc::rc::Rc;
use core::cell::{OnceCell, RefCell, RefMut};

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::transform::builder::Transform2DBuilder;
use kernel_lib::error::{KernelError, KernelResult, LayerReason};
use kernel_lib::gop::console::DISPLAY_BACKGROUND_COLOR;
use kernel_lib::gop::pixel::rc_pixel_writer;
use kernel_lib::layers::window::drawers::cursor::mouse_cursor::MouseCursorDrawer;
use kernel_lib::layers::window::drawers::shape::ShapeWDrawer;
use kernel_lib::layers::window::Window;
use kernel_lib::layers::{frame_buffer_layer_transform, Layers};

pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers<'layer>(OnceCell<Rc<RefCell<Layers<'layer>>>>);


pub const BACKGROUND_LAYER_ID: usize = 0;


pub const MOUSE_LAYER_ID: usize = 1;


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

    add_background_layer(frame_buffer_config, &mut layers);
    add_mouse_layer(frame_buffer_config, &mut layers);

    layers.draw_all_layers_start_at(BACKGROUND_LAYER_ID)?;

    Ok(())
}

fn add_background_layer(frame_buffer_config: FrameBufferConfig, layers: &mut RefMut<Layers>) {
    let transform = frame_buffer_layer_transform(frame_buffer_config);

    let layer = layers.new_layer(transform.clone());

    let window = Window::new(ShapeWDrawer::new(DISPLAY_BACKGROUND_COLOR), transform);


    layer.add_window("background", window);
}


fn add_mouse_layer(frame_buffer_config: FrameBufferConfig, layers: &mut RefMut<Layers>) {
    let layer = layers.new_layer(frame_buffer_layer_transform(frame_buffer_config));

    let transform = Transform2DBuilder::new().build();
    let window = Window::new(MouseCursorDrawer::default(), transform);

    layer.add_window("mouse", window);
}
