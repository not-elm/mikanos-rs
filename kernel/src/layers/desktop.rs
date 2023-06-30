use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};
use kernel_lib::gop::config;
use kernel_lib::gop::console::DISPLAY_BACKGROUND_COLOR;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::layers::frame_buffer_layer_transform;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::multiple_layer::MultipleLayer;
use kernel_lib::layers::shape::shape_drawer::ShapeDrawer;
use kernel_lib::layers::shape::ShapeLayer;

use crate::layers::DESKTOP_LAYER_KEY;

pub(crate) fn desktop() -> LayerKey {
    let transform = frame_buffer_layer_transform(config());
    let mut layers = MultipleLayer::new(transform);

    layers.new_layer(desktop_background());
    layers.new_layer(bottom_bar());

    layers
        .into_enum()
        .into_layer_key(DESKTOP_LAYER_KEY)
}


fn desktop_background() -> LayerKey {
    let transform = frame_buffer_layer_transform(config());
    let shape_drawer = ShapeDrawer::new(config(), DISPLAY_BACKGROUND_COLOR);

    ShapeLayer::new(shape_drawer, transform)
        .into_enum()
        .into_layer_key("Desktop Background")
}


fn bottom_bar() -> LayerKey {
    let screen_size = config().screen_size();
    let pos = Vector2D::new(0, screen_size.height() - 50);
    let size = Size::new(screen_size.width(), 50);
    let transform = Transform2D::new(pos, size);
    let mut layers = MultipleLayer::new(transform);

    layers.new_layer(bottom_bar_background(layers.transform().size()));
    layers.new_layer(bottom_bar_icon_frame(layers.transform().size()));
    layers.new_layer(bottom_bar_icon());

    layers
        .into_enum()
        .into_layer_key("Desktop Bottom Bar")
}


fn bottom_bar_background(size: Size) -> LayerKey {
    let shape_drawer = ShapeDrawer::new(config(), PixelColor::new(1, 18, 17));
    ShapeLayer::new(shape_drawer, Transform2D::new(Vector2D::zeros(), size))
        .into_enum()
        .into_layer_key("Desktop Bottom Bar Background")
}


fn bottom_bar_icon_frame(size: Size) -> LayerKey {
    let shape_drawer = ShapeDrawer::new(config(), PixelColor::new(80, 80, 80));
    ShapeLayer::new(shape_drawer, Transform2D::new(Vector2D::zeros(), Size::new(size.width() / 5, size.height())))
        .into_enum()
        .into_layer_key("Desktop Bottom Bar Icon Frame")
}


fn bottom_bar_icon() -> LayerKey {
    let shape_drawer = ShapeDrawer::new(config(), PixelColor::new(160, 160, 160));
    ShapeLayer::new(shape_drawer, Transform2D::new(Vector2D::new(10, 10), Size::new(30, 30)))
        .into_enum()
        .into_layer_key("Desktop Bottom Bar Background")
}