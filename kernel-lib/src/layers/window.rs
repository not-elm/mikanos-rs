use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use common_lib::{frame_buffer::FrameBufferConfig, transform::transform2d::Transformable2D};

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::console::ConsoleLayer;
use crate::layers::count::CountLayer;
use crate::layers::layer::Layer;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_colors::ShapeColors;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;

use super::close_button::{CloseButtonLayer, CLOSE_BUTTON_HEIGHT, CLOSE_BUTTON_WIDTH};
use super::console::console_colors::ConsoleColors;

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
        multiple_layer.new_layer(count_layer(config, &transform).unwrap());

        Self { multiple_layer }
    }


    pub fn write_count(&mut self, count: usize) {
        self.multiple_layer
            .layers_mut()
            .get_mut(3)
            .unwrap()
            .require_count()
            .unwrap()
            .write_count(count);
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
            ShapeColors::default().change_foreground(PixelColor::window_background()),
        ),
        Transform2D::new(
            Vector2D::zeros(),
            Size::new(transform.size().width() - 1, transform.size().height() - 1),
        ),
    )
    .into_enum()
}


fn toolbar_layer(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    let toolbar_transform = Transform2D::new(
        Vector2D::new(3, 3),
        Size::new(transform.size().width() - 6, 24),
    );
    let mut layer = MultipleLayer::new(toolbar_transform);

    layer.new_layer(
        ShapeLayer::new(
            ShapeDrawer::new(
                config,
                ShapeColors::new(PixelColor::new(0x00, 0x00, 0x84), None),
            ),
            Transform2D::new(Vector2D::zeros(), layer.rect().size()),
        )
        .into_enum(),
    );

    layer.new_layer(toolbar_title_layer(config));
    layer.new_layer(toolbar_close_button(config, layer.transform_ref()));

    layer.into_enum()
}


fn toolbar_title_layer(config: FrameBufferConfig) -> Layer {
    let mut text = ConsoleLayer::new(config, ConsoleColors::default().change_background(PixelColor::new(0x00, 0x00, 0x84)), Vector2D::new(24, 4), Size::new(12, 1));

    text.update_string("Hello Window")
        .unwrap();

    text.into_enum()
}


fn toolbar_close_button(config: FrameBufferConfig, transform: &Transform2D) -> Layer {
    CloseButtonLayer::new(
        config,
        Vector2D::new(
            transform.size().width() - CLOSE_BUTTON_WIDTH - 5,
            (transform.size().height() - CLOSE_BUTTON_HEIGHT) / 2,
        ),
    )
    .into_enum()
}


fn count_layer(config: FrameBufferConfig, window_transform: &Transform2D) -> KernelResult<Layer> {
    const TOOLBAR_HEIGHT: usize = 24;

    let size = window_transform.size() / 2;
    let x = (window_transform
        .size()
        .width()
        - size.width())
        / 2;

    let y: usize = (window_transform
        .size()
        .height()
        - size.height()
        - TOOLBAR_HEIGHT)
        / 2
        + TOOLBAR_HEIGHT;
    let pos = Vector2D::new(x, y);
    let count = CountLayer::new(config, Transform2D::new(pos, size))?;

    Ok(count.into_enum())
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::transform2d::Transform2D;

    use crate::layers::window::WindowLayer;

    #[test]
    fn it_update_count_not_panic() {
        let mut window = WindowLayer::new(
            FrameBufferConfig::mock(),
            Transform2D::new(Vector2D::zeros(), Size::new(300, 300)),
        );

        window.write_count(1);
    }
}
