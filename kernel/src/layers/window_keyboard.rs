use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::error::KernelResult;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::text_box::TextBoxLayer;
use kernel_lib::layers::window::WindowLayer;

use crate::layers::{KEYBOARD_TEXT, WINDOW_KEYBOARD};

pub(crate) fn window_keyboard(config: FrameBufferConfig) -> KernelResult<LayerKey> {
    let pos = Vector2D::new(500, 200);
    let size = Size::new(180, 55);

    let transform = Transform2D::new(pos, size);
    Ok(WindowLayer::new(config, transform, "Text Box")
        .new_layer(keyboard_text_box(config))?
        .into_enum()
        .into_layer_key(WINDOW_KEYBOARD))
}


fn keyboard_text_box(config: FrameBufferConfig) -> LayerKey {
    let pos = Vector2D::new(5, 0);
    let size = Size::new(170, 20);

    TextBoxLayer::new(config, Transform2D::new(pos, size))
        .into_enum()
        .into_layer_key(KEYBOARD_TEXT)
}
