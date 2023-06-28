use core::fmt::Write;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::LAYERS;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::colors::TextColors;
use crate::layers::text::TextLayer;
use crate::timer::handler::TimeHandle;

const TEXT_LAYER_KEY: &str = "Text Box Text";
const CURSOR_LAYER_KEY: &str = "Text Box Cursor";

#[derive(Delegate)]
pub struct TextBoxLayer {
    #[to(LayerUpdatable, Transformable2D, LayerFindable)]
    pub layers: MultipleLayer,

    visible_text_cursor: bool,

    _cursor_handle: TimeHandle,
}


impl TextBoxLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> Self {
        let (layers, cursor_handle) = text_box_layers(config, transform);

        Self {
            layers,
            visible_text_cursor: true,
            _cursor_handle: cursor_handle,
        }
    }


    #[inline(always)]
    pub fn delete_last(&mut self) {
        self
            .text_layer()
            .delete_last();

        self.update_text_cursor_pos();
    }


    pub fn update_text_cursor_color(&mut self) {
        self.visible_text_cursor = !self.visible_text_cursor;
        if self.visible_text_cursor {
            self.text_cursor_layer().set_color(PixelColor::black());
        } else {
            self.text_cursor_layer().set_color(PixelColor::white());
        }
    }


    #[inline(always)]
    pub fn into_enum(self) -> Layer {
        Layer::TextBox(self)
    }


    fn update_text_cursor_pos(&mut self) {
        let pos = self
            .text_layer()
            .text_cursor_pos();
        let pos = self.pos() + pos + Vector2D::new(7, 3);

        self.text_cursor_layer()
            .move_to(pos)
    }


    #[inline(always)]
    fn text_cursor_layer(&mut self) -> &mut ShapeLayer {
        self
            .layers
            .force_find_by_key_mut(CURSOR_LAYER_KEY)
            .require_shape()
            .unwrap()
    }


    #[inline(always)]
    fn text_layer(&mut self) -> &mut TextLayer {
        self
            .layers
            .force_find_by_key_mut(TEXT_LAYER_KEY)
            .require_text()
            .unwrap()
    }
}


impl Write for TextBoxLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self
            .text_layer()
            .write_str(s)?;

        self.update_text_cursor_pos();

        Ok(())
    }
}


fn text_box_layers(config: FrameBufferConfig, transform: Transform2D) -> (MultipleLayer, TimeHandle) {
    let mut layers = MultipleLayer::new(transform);
    layers.new_layer(inner_shadow(
        config,
        layers.transform(),
    ));
    layers.new_layer(drop_shadow(config, layers.transform()));
    layers.new_layer(background(config, layers.transform()));
    layers.new_layer(text(config));
    let (cursor, handle) = cursor(config);
    layers.new_layer(cursor);

    (layers, handle)
}


fn inner_shadow(config: FrameBufferConfig, root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::zeros(), root_transform.size());
    let layer = ShapeLayer::new(
        ShapeDrawer::new(config, PixelColor::new(0x84, 0x84, 0x84)),
        transform,
    );

    layer
        .into_enum()
        .into_layer_key("Text Box Background")
}


fn drop_shadow(config: FrameBufferConfig, root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::unit(), root_transform.size());
    let layer = ShapeLayer::new(
        ShapeDrawer::new(config, PixelColor::new(0xC6, 0xC6, 0xC6)),
        transform,
    );

    layer
        .into_enum()
        .into_layer_key("Text Box Shadow")
}


fn background(config: FrameBufferConfig, root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::unit(), root_transform.size() - 2);
    let layer = ShapeLayer::new(ShapeDrawer::new(config, PixelColor::white()), transform);

    layer
        .into_enum()
        .into_layer_key("Text Box Outline")
}


fn text(config: FrameBufferConfig) -> LayerKey {
    let pos = Vector2D::new(5, 2);
    let text_frame_size = Size::new(20, 1);
    let colors = TextColors::default()
        .change_foreground(PixelColor::black())
        .change_background(PixelColor::white());

    TextLayer::new(config, pos, text_frame_size, colors)
        .into_enum()
        .into_layer_key(TEXT_LAYER_KEY)
}


fn cursor(config: FrameBufferConfig) -> (LayerKey, TimeHandle) {
    let cursor_handle = TimeHandle::start_dispatch_on_main(70, || {
        LAYERS
            .lock()
            .update_layer("WINDOW TEXT", |layer| {
                layer
                    .require_text_box()
                    .unwrap()
                    .update_text_cursor_color();
            })
            .unwrap();
    });

    let transform = Transform2D::new(Vector2D::new(3, 3), Size::new(1, 14));
    let layer = ShapeLayer::new(ShapeDrawer::new(config, PixelColor::black()), transform);
    let layer_key = Layer::Shape(layer)
        .into_layer_key(CURSOR_LAYER_KEY);

    (layer_key, cursor_handle)
}
