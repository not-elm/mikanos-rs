use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use common_lib::{frame_buffer::FrameBufferConfig, transform::transform2d::Transformable2D};

use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::{LayerFindable, MultipleLayer};
use crate::layers::shape::shape_colors::ShapeColors;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::window::toolbar::ToolbarLayer;

pub(crate) mod toolbar;


#[derive(Delegate)]
pub struct WindowLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    layers: MultipleLayer,
}


impl WindowLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> Self {
        let mut multiple_layer = MultipleLayer::new(transform.clone());

        multiple_layer.new_layer(shadow_layer(config, &transform));
        multiple_layer.new_layer(window_background_layer(config, &transform));
        multiple_layer.new_layer(toolbar_layer(config, &transform));
        // multiple_layer.new_layer(count_layer(config, &transform).unwrap());

        Self {
            layers: multiple_layer,
        }
    }


    pub fn new_layer(mut self, layer: LayerKey) -> Self {
        self.layers.new_layer(layer);

        self
    }


    pub const fn into_enum(self) -> Layer {
        Layer::Window(self)
    }
}


fn shadow_layer(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
    ShapeLayer::new(
        ShapeDrawer::new(
            config,
            ShapeColors::default().change_foreground(PixelColor::black()),
        ),
        Transform2D::new(Vector2D::zeros(), transform.size()),
    )
    .into_enum()
    .into_layer_key("window shadow")
}


fn window_background_layer(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
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
    .into_layer_key("window background")
}


fn toolbar_layer(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
    let toolbar_transform = Transform2D::new(
        Vector2D::new(3, 3),
        Size::new(transform.size().width() - 6, 24),
    );

    ToolbarLayer::new(config, toolbar_transform)
        .into_enum()
        .into_layer_key("window toolbar")
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
