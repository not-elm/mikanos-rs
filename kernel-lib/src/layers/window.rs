use alloc::format;

use auto_delegate::Delegate;

use common_lib::{frame_buffer::FrameBufferConfig, transform::transform2d::Transformable2D};
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;

use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::console::console_colors::ConsoleColors;
use crate::layers::console::ConsoleLayer;
use crate::layers::layer::Layer;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_colors::ShapeColors;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;

use super::close_button::{CloseButtonLayer, CLOSE_BUTTON_WIDTH, CLOSE_BUTTON_HEIGHT};

#[derive(Delegate)]
pub struct WindowLayer {
    #[to(Transformable2D, LayerUpdatable)]
    multiple_layer: MultipleLayer,
}


impl WindowLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> Self {
        let mut multiple_layer = MultipleLayer::new(transform.clone());

        multiple_layer.new_layer(shadow_layer(config, &transform));
        multiple_layer.new_layer(window_background_layer(config, &transform));
        multiple_layer.new_layer(toolbar_layer(config, &transform));
        multiple_layer.new_layer(count_text_layer(config));

        Self { multiple_layer }
    }


    pub fn write_count(&mut self, count: usize) {
        self.multiple_layer
            .layers_mut()
            .get_mut(3)
            .unwrap()
            .require_console()
            .unwrap()
            .update_string(format!("{}", count).as_str())
            .unwrap();
    }


    pub const fn into_enum(self) -> Layer {
        Layer::Window(self)
    }
}


fn shadow_layer(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::default().change_foreground(PixelColor::black()),
        ),
        Transform2D::new(Vector2D::zeros(), transform.size()),
    )
    .into_enum()
}


fn window_background_layer(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::default().change_foreground(PixelColor::new(0xC6, 0xC6, 0xC6)),
        ),
        Transform2D::new(
            Vector2D::zeros(),
            Size::new(transform.size().width() - 1, transform.size().height() - 1),
        ),
    )
    .into_enum()
}


fn toolbar_layer(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    let toolbar_transform =  Transform2D::new(
        Vector2D::new(3, 3),
        Size::new(transform.size().width() - 6, 24),
    );
    let mut layer = MultipleLayer::new(toolbar_transform);

    layer.new_layer(ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::new(PixelColor::new(0x00, 0x00, 0x84), None),
        ),
        Transform2D::new(Vector2D::zeros(), layer.rect().size()),
    )
    .into_enum());

    layer.new_layer(toolbar_title_layer(config));
    layer.new_layer(toolbar_close_button(config, layer.transform_ref()));
    
    layer.into_enum()
}


fn toolbar_title_layer(config: FrameBufferConfig) -> Layer {
    let mut text = ConsoleLayer::new(
        config,
        Transform2D::new(Vector2D::new(24, 4), Size::new(200, 16)),
        ConsoleColors::new(PixelColor::white(), PixelColor::new(0x00, 0x00, 0x84)),
    );

    text.update_string("Hello Window")
        .unwrap();

    text.into_enum()
}


fn toolbar_close_button(config: FrameBufferConfig, transform: &Transform2D) -> Layer{
    CloseButtonLayer::new(config, Vector2D::new(
        transform.size().width() - CLOSE_BUTTON_WIDTH - 5,  
    (transform.size().height() - CLOSE_BUTTON_HEIGHT) / 2)
    )
        .into_enum()
}


fn count_text_layer(config: FrameBufferConfig) -> Layer {
    ConsoleLayer::new(
        config,
        Transform2D::new(Vector2D::new(100, 100), Size::new(100, 20)),
        ConsoleColors::new(PixelColor::white(), PixelColor::black()),
    )
    .into_enum()
}
