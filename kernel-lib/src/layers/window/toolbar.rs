use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::close_button::{CloseButtonLayer, CLOSE_BUTTON_HEIGHT, CLOSE_BUTTON_WIDTH};
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_colors::ShapeColors;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::console_colors::TextColors;
use crate::layers::text::TextLayer;

#[derive(Delegate)]
pub struct ToolbarLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    layers: MultipleLayer,
}


impl ToolbarLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D, title: &str) -> Self {
        Self {
            layers: toolbar_layer(config, transform, title),
        }
    }


    pub fn into_enum(self) -> Layer {
        Layer::Toolbar(self)
    }
}


fn toolbar_layer(config: FrameBufferConfig, transform: Transform2D, title: &str) -> MultipleLayer {
    let mut layer = MultipleLayer::new(transform.clone());

    layer.new_layer(toolbar_background_layer(config, transform));
    layer.new_layer(toolbar_title_layer(config, title));
    layer.new_layer(toolbar_close_button(config, layer.transform_ref()));

    layer
}


fn toolbar_background_layer(config: FrameBufferConfig, transform: Transform2D) -> LayerKey {
    ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::new(PixelColor::new(0x00, 0x00, 0x84), None),
        ),
        transform,
    )
    .into_enum()
    .into_layer_key("window toolbar")
}


fn toolbar_title_layer(config: FrameBufferConfig, title: &str) -> LayerKey {
    let mut text = TextLayer::new(
        config,
        Vector2D::new(24, 4),
        Size::new(12, 1),
        TextColors::default().change_background(PixelColor::new(0x00, 0x00, 0x84)),
    );

    text.update_string(title)
        .unwrap();

    text.into_enum()
        .into_layer_key("toolbar title")
}


fn toolbar_close_button(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
    CloseButtonLayer::new(
        config,
        Vector2D::new(
            transform.size().width() - CLOSE_BUTTON_WIDTH - 5,
            (transform.size().height() - CLOSE_BUTTON_HEIGHT) / 2,
        ),
    )
    .into_enum()
    .into_layer_key("toolbar close button")
}
