use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use kernel_lib::gop;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::text::{config, TextLayer};

use crate::layers::CONSOLE_LAYER_KEY;

pub(crate) fn console() -> LayerKey {
    let config = config::Builder::new()
        .set_scrollable()
        .build();

    TextLayer::new(gop::config(), Vector2D::zeros(), Size::new(50, 10), config)
        .unwrap()
        .into_enum()
        .into_layer_key(CONSOLE_LAYER_KEY)
}
