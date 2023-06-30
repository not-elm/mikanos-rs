use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::terminal::TerminalLayer;

use crate::layers::TERMINAL_LAYER_KEY;

pub(crate) fn terminal() -> LayerKey {
    let pos = Vector2D::new(100, 200);
    let size = Size::new(500, 16 * 20 + 10 + 17);
    let transform = Transform2D::new(pos, size);

    TerminalLayer::new(transform)
        .into_enum()
        .into_layer_key(TERMINAL_LAYER_KEY)
}
