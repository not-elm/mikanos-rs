use core::fmt::Write;

use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;

use crate::gop::config;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::LayerFindable;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text_box::TextBoxLayer;
use crate::layers::window::WindowLayer;

const TEXT_LAYER_KEY: &str = "Terminal Text";

#[derive(Delegate)]
pub struct TerminalLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    window: WindowLayer,

}


impl TerminalLayer {
    pub fn new(transform: Transform2D) -> TerminalLayer {
        let window = WindowLayer::new_dark_color("", transform)
            .then_add(text_background)
            .unwrap()
            .then_add(text)
            .unwrap();

        Self {
            window,
        }
    }


    pub fn into_enum(self) -> Layer {
        Layer::Terminal(self)
    }
}


impl Write for TerminalLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.window
            .find_by_key_mut(TEXT_LAYER_KEY)
            .unwrap()
            .require_text_box()
            .unwrap()
            .write_str(s)
    }
}


fn text_background(size: Size) -> LayerKey {
    let drawer = ShapeDrawer::new(config(), PixelColor::black());
    ShapeLayer::new(drawer, Transform2D::new(Vector2D::zeros(), size))
        .into_enum()
        .into_layer_key("Terminal Text Background")
}


fn text(size: Size) -> LayerKey {
    let pos = Vector2D::zeros();
    let text_frame_size = size / Size::new(8, 16);

    let mut text = TextBoxLayer::new_dark(Transform2D::new(pos, text_frame_size));
    text.write_str(">").unwrap();

    text
        .into_enum()
        .into_layer_key(TEXT_LAYER_KEY)
}


