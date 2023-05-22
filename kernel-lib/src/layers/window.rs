use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
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

#[derive(Delegate)]
pub struct WindowLayer {
    #[to(Transformable2D, LayerUpdatable)]
    multiple_layer: MultipleLayer,
}


impl WindowLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> Self {
        let mut multiple_layer = MultipleLayer::new(transform.clone());

        multiple_layer.new_layer(window_background_layer(config, &transform));
        multiple_layer.new_layer(toolbar_layer(config, &transform));

        Self { multiple_layer }
    }


    pub const fn into_enum(self) -> Layer {
        Layer::Window(self)
    }
}


fn window_background_layer(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::default().change_foreground(PixelColor::yellow()),
        ),
        Transform2D::new(Vector2D::zeros(), transform.size()),
    )
        .into_enum()
}


fn toolbar_layer(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    let height = transform.size().height() as f64 * 0.1;
    let height = height as usize;

    ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::new(PixelColor::new(0x33, 0x33, 0x33), None),
        ),
        Transform2D::new(
            Vector2D::zeros(),
            Size::new(transform.size().width(), height),
        ),
    )
        .into_enum()
}


fn count_text_layer(config: FrameBufferConfig) -> Layer {
    ConsoleLayer::new(config, ConsoleColors::default()).into_enum()
}
