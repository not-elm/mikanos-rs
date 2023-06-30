use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;

use crate::gop::config;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::text::colors::TextColors;
use crate::layers::text::TextLayer;
use crate::layers::window::WindowLayer;

#[derive(Delegate)]
pub struct TerminalLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    window: WindowLayer,
}


impl TerminalLayer {
    pub fn new(transform: Transform2D) -> TerminalLayer {
        let window = WindowLayer::new_dark_color("", transform)
            .then_add(text)
            .unwrap();

        Self {
            window
        }
    }


    pub fn into_enum(self) -> Layer {
        Layer::Terminal(self)
    }
}


fn text(size: Size) -> LayerKey {
    let pos = Vector2D::zeros();
    let text_frame_size =  size / Size::new(8, 16);
    let colors = TextColors::new(PixelColor::white(), PixelColor::black());

    TextLayer::new(config(), pos, text_frame_size, colors)
        .into_enum()
        .into_layer_key("Terminal Window")
}