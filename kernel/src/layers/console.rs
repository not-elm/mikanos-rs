use crate::layers::CONSOLE_LAYER_KEY;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::text::colors::TextColors;
use kernel_lib::layers::text::TextLayer;

pub(crate) fn console(config: FrameBufferConfig) -> LayerKey {
    TextLayer::new(
        config,
        Vector2D::zeros(),
        Size::new(50, 10),
        TextColors::default(),
        true,
        None
    )
        .unwrap()
    .into_enum()
    .into_layer_key(CONSOLE_LAYER_KEY)
}
