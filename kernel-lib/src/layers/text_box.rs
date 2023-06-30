use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::gop::config;
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

#[derive(Delegate)]
pub struct TextBoxLayer {
    #[to(LayerUpdatable, Transformable2D, LayerFindable)]
    pub layers: MultipleLayer,
    text_cursor_key: String,
    _cursor_handle: TimeHandle,
}


impl TextBoxLayer {
    pub fn new_light(
        transform: Transform2D,
    ) -> Self {
        Self::new(
            transform,
            TextColors::new(PixelColor::black(), PixelColor::white()),
            true
        )
    }


    pub fn new_dark(
        transform: Transform2D,
    ) -> Self {
        Self::new(
            transform,
            TextColors::new(PixelColor::white(), PixelColor::black()),
            false
        )
    }


    pub fn new(
        transform: Transform2D,
        colors: TextColors,
        with_shadow: bool,
    ) -> Self {
        let (layers, text_cursor_key, cursor_handle) = text_box_layers(transform, colors, with_shadow);

        Self {
            layers,
            text_cursor_key,
            _cursor_handle: cursor_handle,
        }
    }


    #[inline(always)]
    pub fn delete_last(&mut self) {
        self.text_layer()
            .delete_last();

        self.update_text_cursor_pos();
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
        self.layers
            .force_find_by_key_mut(&self.text_cursor_key)
            .require_shape()
            .unwrap()
    }


    #[inline(always)]
    fn text_layer(&mut self) -> &mut TextLayer {
        self.layers
            .force_find_by_key_mut(TEXT_LAYER_KEY)
            .require_text()
            .unwrap()
    }
}


impl Write for TextBoxLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.text_layer()
            .write_str(s)?;

        self.update_text_cursor_pos();

        Ok(())
    }
}


fn text_box_layers(
    transform: Transform2D,
    colors: TextColors,
    with_shadow: bool,
) -> (MultipleLayer, String, TimeHandle) {
    let mut layers = MultipleLayer::new(transform);
    if with_shadow {
        layers.new_layer(inner_shadow(layers.transform()));
        layers.new_layer(drop_shadow(layers.transform()));
        layers.new_layer(background(layers.transform(), 1,colors.background()));
    } else {
        layers.new_layer(background(layers.transform(), 0 ,colors.background()));
    }

    layers.new_layer(text(colors));
    let (cursor, handle) = cursor(colors);
    let cursor_key = cursor.key().to_string();
    layers.new_layer(cursor);

    (layers, cursor_key, handle)
}


fn inner_shadow(root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::zeros(), root_transform.size());
    let layer = ShapeLayer::new(
        ShapeDrawer::new(config(), PixelColor::new(0x84, 0x84, 0x84)),
        transform,
    );

    layer
        .into_enum()
        .into_layer_key("Text Box Inner Shadow")
}


fn drop_shadow(root_transform: Transform2D) -> LayerKey {
    let transform = Transform2D::new(Vector2D::unit(), root_transform.size());
    let layer = ShapeLayer::new(
        ShapeDrawer::new(config(), PixelColor::new(0xC6, 0xC6, 0xC6)),
        transform,
    );

    layer
        .into_enum()
        .into_layer_key("Text Box Shadow")
}


fn background(root_transform: Transform2D, sub_size: usize, color: PixelColor) -> LayerKey {
    let transform = Transform2D::new(Vector2D::unit(), root_transform.size() - sub_size);
    let layer = ShapeLayer::new(ShapeDrawer::new(config(), color), transform);

    layer
        .into_enum()
        .into_layer_key("Text Box Background")
}


fn text(colors: TextColors) -> LayerKey {
    let pos = Vector2D::new(5, 2);
    let text_frame_size = Size::new(20, 1);

    TextLayer::new(config(), pos, text_frame_size, colors)
        .into_enum()
        .into_layer_key(TEXT_LAYER_KEY)
}


fn cursor(colors: TextColors) -> (LayerKey, TimeHandle) {
    let visible = AtomicBool::new(true);
    static ID: AtomicUsize = AtomicUsize::new(0);
    let key = format!("Text Box Cursor{}", ID.fetch_add(1, Ordering::Relaxed));

    let key_copy = key.clone();
    let cursor_handle = TimeHandle::start_dispatch_on_main(70, move || {
        LAYERS
            .lock()
            .update_layer(&key_copy, |layer| {
                let text = layer.require_shape().unwrap();
                if visible.fetch_not(Ordering::Relaxed) {
                    text.set_color(colors.foreground())
                } else {
                    text.set_color(colors.background())
                }
            })
            .unwrap();
    });

    let transform = Transform2D::new(Vector2D::new(3, 3), Size::new(3, 14));
    let layer = ShapeLayer::new(ShapeDrawer::new(config(), colors.foreground()), transform);
    let layer_key = Layer::Shape(layer).into_layer_key(&key);

    (layer_key, cursor_handle)
}
