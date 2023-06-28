use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::close_button::{CLOSE_BUTTON_HEIGHT, CLOSE_BUTTON_WIDTH, CloseButtonLayer};
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::{LayerFindable, MultipleLayer};
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::colors::TextColors;
use crate::layers::text::TextLayer;

const BACKGROUND_LAYER_KAY: &str = "Window Toolbar Background";
const TITLE_LAYER_KAY: &str = "Window Toolbar Title";
const DEACTIVATE_COLORS: TextColors = TextColors::new(
    PixelColor::new(0xEB, 0xEB, 0xE4),
    PixelColor::new(0x84, 0x84, 0x84),
);
const ACTIVE_COLORS: TextColors = TextColors::new(
    PixelColor::white(),
    PixelColor::new(0x00, 0x00, 0x84),
);


#[derive(Delegate)]
pub struct ToolbarLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    layers: MultipleLayer,
}


impl ToolbarLayer {
    #[inline]
    pub fn new(config: FrameBufferConfig, transform: Transform2D, title: &str) -> Self {
        Self {
            layers: toolbar_layer(config, transform, title),
        }
    }

    #[inline]
    pub fn activate(&mut self) -> KernelResult {
        self.layers
            .find_by_key_mut(BACKGROUND_LAYER_KAY)
            .unwrap()
            .require_shape()
            .unwrap()
            .set_color(*ACTIVE_COLORS.background());

        self.layers
            .force_find_by_key_mut(TITLE_LAYER_KAY)
            .require_text()
            .unwrap()
            .change_colors(ACTIVE_COLORS)
    }


    #[inline]
    pub fn deactivate(&mut self) -> KernelResult {
        self.layers
            .find_by_key_mut(BACKGROUND_LAYER_KAY)
            .unwrap()
            .require_shape()
            .unwrap()
            .set_color(*DEACTIVATE_COLORS.background());

        self.layers
            .force_find_by_key_mut(TITLE_LAYER_KAY)
            .require_text()
            .unwrap()
            .change_colors(DEACTIVATE_COLORS)
    }


    #[inline]
    pub fn into_enum(self) -> Layer {
        Layer::Toolbar(self)
    }
}


fn toolbar_layer(config: FrameBufferConfig, transform: Transform2D, title: &str) -> MultipleLayer {
    let mut layer = MultipleLayer::new(transform.clone());

    layer.new_layer(toolbar_background_layer(config, transform));
    layer.new_layer(toolbar_title_layer(config, title));
    layer.new_layer(toolbar_close_button(config, &layer.transform()));

    layer
}


fn toolbar_background_layer(config: FrameBufferConfig, transform: Transform2D) -> LayerKey {
    ShapeLayer::new(
        ShapeDrawer::new(config, *DEACTIVATE_COLORS.background()),
        transform,
    )
        .into_enum()
        .into_layer_key(BACKGROUND_LAYER_KAY)
}


fn toolbar_title_layer(config: FrameBufferConfig, title: &str) -> LayerKey {
    let mut text = TextLayer::new(
        config,
        Vector2D::new(24, 4),
        Size::new(12, 1),
        DEACTIVATE_COLORS,
    );

    text.update_string(title)
        .unwrap();

    text.into_enum()
        .into_layer_key(TITLE_LAYER_KAY)
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
