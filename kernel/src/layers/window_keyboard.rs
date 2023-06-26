use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};
use kernel_lib::error::KernelResult;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::multiple_layer::MultipleLayer;
use kernel_lib::layers::shape::shape_drawer::ShapeDrawer;
use kernel_lib::layers::shape::ShapeLayer;
use kernel_lib::layers::text::console_colors::TextColors;
use kernel_lib::layers::text::TextLayer;
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

    let mut layers = MultipleLayer::new(Transform2D::new(pos, size));
    layers.new_layer(text_box_inner_shadow(
        config,
        layers.transform_ref().clone(),
    ));
    layers.new_layer(text_box_drop_shadow(config, layers.transform_ref().clone()));
    layers.new_layer(text_box_background(config, layers.transform_ref().clone()));
    layers.new_layer(keyboard_text(config));

    layers
        .into_enum()
        .into_layer_key("Keyboard Text Box")
}


fn text_box_inner_shadow(config: FrameBufferConfig, root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::zeros(), root_transform.size());
    let layer = ShapeLayer::new(
        ShapeDrawer::new(config, PixelColor::new(0x84, 0x84, 0x84)),
        transform,
    );

    layer
        .into_enum()
        .into_layer_key("Text Box Background")
}


fn text_box_drop_shadow(config: FrameBufferConfig, root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::unit(), root_transform.size());
    let layer = ShapeLayer::new(
        ShapeDrawer::new(config, PixelColor::new(0xC6, 0xC6, 0xC6)),
        transform,
    );

    layer
        .into_enum()
        .into_layer_key("Text Box Shadow")
}


fn text_box_background(config: FrameBufferConfig, root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::unit(), root_transform.size() - 2);
    let layer = ShapeLayer::new(ShapeDrawer::new(config, PixelColor::white()), transform);

    layer
        .into_enum()
        .into_layer_key("Text Box Outline")
}


fn keyboard_text(config: FrameBufferConfig) -> LayerKey {
    let pos = Vector2D::new(5, 2);
    let text_frame_size = Size::new(20, 1);
    let colors = TextColors::default()
        .change_foreground(PixelColor::black())
        .change_background(PixelColor::white());

    TextLayer::new(config, pos, text_frame_size, colors)
        .into_enum()
        .into_layer_key(KEYBOARD_TEXT)
}
