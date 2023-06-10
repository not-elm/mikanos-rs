use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::gop::console::DISPLAY_BACKGROUND_COLOR;
use kernel_lib::layers::frame_buffer_layer_transform;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::shape::shape_colors::ShapeColors;
use kernel_lib::layers::shape::shape_drawer::ShapeDrawer;
use kernel_lib::layers::shape::ShapeLayer;
use crate::layers::BACKGROUND_LAYER_KEY;

pub(crate) fn screen_background(config: FrameBufferConfig) -> LayerKey {
    let transform = frame_buffer_layer_transform(config);
    let colors = ShapeColors::default().change_foreground(DISPLAY_BACKGROUND_COLOR);
    let shape_drawer = ShapeDrawer::new(config, colors);

    ShapeLayer::new(shape_drawer, transform)
        .into_enum()
        .into_layer_key(BACKGROUND_LAYER_KEY)
}
