use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::layers::console::console_colors::TextColors;
use kernel_lib::layers::console::TextLayer;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::window::WindowLayer;

use crate::layers::{KEYBOARD_TEXT, WINDOW_KEYBOARD};

pub(crate) fn window_keyboard(config: FrameBufferConfig) -> LayerKey {
    let pos = Vector2D::new(350, 200);
    let size = Size::new(300, 52);

    let transform = Transform2D::new(pos, size);
    WindowLayer::new(config, transform)
        .new_layer(keyboard_text_box(config))
        .into_enum()
        .into_layer_key(WINDOW_KEYBOARD)
}


fn keyboard_text_box(config: FrameBufferConfig) -> LayerKey {
    let pos = Vector2D::new(4, 24);
    let text_frame_size = Size::new(20, 1);
    let colors = TextColors::default()
        .change_foreground(PixelColor::black())
        .change_background(PixelColor::window_background());

    TextLayer::new(config, pos, text_frame_size, colors)
        .into_enum()
        .into_layer_key(KEYBOARD_TEXT)
}
