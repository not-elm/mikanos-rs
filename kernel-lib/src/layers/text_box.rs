use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop;
use crate::gop::config;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::colors::TextColors;
use crate::layers::text::config::TextConfig;
use crate::layers::text::TextLayer;
use crate::layers::LAYERS;
use crate::timer::handler::TimeHandle;

const TEXT_LAYER_KEY: &str = "Text Box Text";

#[derive(Delegate)]
pub struct TextBoxLayer {
    #[to(LayerUpdatable, Transformable2D, LayerFindable)]
    pub layers: MultipleLayer,
    text_cursor_key: String,
    _cursor_handle: TimeHandle,
}


const PADDING_TEXT_CURSOR: Vector2D<usize> = Vector2D::new(5, 3);

impl TextBoxLayer {
    pub fn new_light(transform: Transform2D, mut config: TextConfig) -> Self {
        config.colors = TextColors::new(PixelColor::black(), PixelColor::white());
        Self::new(transform, true, config)
    }


    pub fn new_dark(transform: Transform2D, mut config: TextConfig) -> Self {
        config.colors = TextColors::new(PixelColor::white(), PixelColor::black());

        Self::new(transform, false, config)
    }


    pub fn new(transform: Transform2D, with_shadow: bool, config: TextConfig) -> Self {
        let colors = config.colors;

        let (layers, text_cursor_key) = text_box_layers(transform, with_shadow, config);
        let cursor_handle = start_cursor_timer(text_cursor_key.clone(), colors);

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


    #[inline]
    pub fn history_up(&mut self) -> KernelResult {
        self.text_layer()
            .history_up()?;
        self.update_text_cursor_pos();

        Ok(())
    }


    #[inline]
    pub fn history_down(&mut self) -> KernelResult {
        self.text_layer()
            .history_down()?;

        self.update_text_cursor_pos();

        Ok(())
    }


    fn update_text_cursor_pos(&mut self) {
        let pos = self
            .text_layer()
            .text_cursor_pos();
        let pos = self.pos() + pos + PADDING_TEXT_CURSOR;

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
    with_shadow: bool,
    config: TextConfig,
) -> (MultipleLayer, String) {
    let size = transform.size();
    let mut layers = MultipleLayer::new(transform);
    if with_shadow {
        layers.new_layer(inner_shadow(layers.transform()));
        layers.new_layer(drop_shadow(layers.transform()));
        layers.new_layer(background(
            layers.transform(),
            1,
            config.colors.background(),
        ));
    } else {
        layers.new_layer(background(
            layers.transform(),
            0,
            config.colors.background(),
        ));
    }

    layers.new_layer(text(size, config.clone()));
    let cursor = cursor(config);
    let cursor_key = cursor.key().to_string();
    layers.new_layer(cursor);

    (layers, cursor_key)
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


fn text(root_size: Size, config: TextConfig) -> LayerKey {
    let pos = Vector2D::new(5, 2);
    let text_frame_size = (root_size - Size::new(10, 4)).unwrap() / Size::new(8, 16);

    TextLayer::new(gop::config(), pos, text_frame_size, config)
        .unwrap()
        .into_enum()
        .into_layer_key(TEXT_LAYER_KEY)
}


fn cursor(config: TextConfig) -> LayerKey {
    static ID: AtomicUsize = AtomicUsize::new(0);
    let key = format!("Text Box Cursor{}", ID.fetch_add(1, Ordering::Relaxed));

    let pos = if config.prefix.is_some() {
        PADDING_TEXT_CURSOR + Vector2D::new(8, 0)
    } else {
        PADDING_TEXT_CURSOR
    };
    let transform = Transform2D::new(pos, Size::new(3, 14));
    let layer = ShapeLayer::new(
        ShapeDrawer::new(gop::config(), config.colors.foreground()),
        transform,
    );

    Layer::Shape(layer).into_layer_key(&key)
}


fn start_cursor_timer(key: String, colors: TextColors) -> TimeHandle {
    let visible = AtomicBool::new(true);
    TimeHandle::start_dispatch_on_main(70, move || {
        LAYERS
            .lock()
            .update_layer(&key, |layer| {
                let text = layer.require_shape().unwrap();
                if visible.fetch_not(Ordering::Relaxed) {
                    text.set_color(colors.foreground())
                } else {
                    text.set_color(colors.background())
                }
            })
            .unwrap();
    })
}
