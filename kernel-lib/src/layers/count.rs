use alloc::format;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::console::console_colors::ConsoleColors;
use crate::layers::console::ConsoleLayer;
use crate::layers::layer::Layer;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_colors::ShapeColors;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;

#[derive(Delegate)]
pub struct CountLayer {
    #[to(Transformable2D, LayerUpdatable)]
    layers: MultipleLayer,
}


impl CountLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> KernelResult<Self> {
        Ok(Self {
            layers: count_layers(config, transform)?,
        })
    }


    pub fn write_count(&mut self, count: usize) {
        self.layers
            .layers_mut()
            .get_mut(1)
            .unwrap()
            .require_console()
            .unwrap()
            .update_string(format!("{}", count).as_str())
            .unwrap();
    }


    pub fn into_enum(self) -> Layer {
        Layer::Count(self)
    }
}


fn count_layers(config: FrameBufferConfig, transform: Transform2D) -> KernelResult<MultipleLayer> {
    let mut layers = MultipleLayer::new(transform);

    layers.new_layer(background_layer(config, layers.transform_ref()));
    layers.new_layer(text_layer(config, layers.transform_ref())?);

    Ok(layers)
}


fn background_layer(config: FrameBufferConfig, root_transform: &Transform2D) -> Layer {
    ShapeLayer::new(
        ShapeDrawer::new(config, ShapeColors::new(PixelColor::black(), None)),
        Transform2D::new(Vector2D::zeros(), root_transform.size()),
    )
    .into_enum()
}


fn text_layer(config: FrameBufferConfig, root_transform: &Transform2D) -> KernelResult<Layer> {
    const PADDING: usize = 5;

    let root_size = root_transform.size();
    let pos = Vector2D::new(root_size.width() / 2 - 64, root_size.height() / 2 - 8);

    let mut text = ConsoleLayer::new(
        config,
        Transform2D::new(pos, Size::new(100, 16)),
        ConsoleColors::new(PixelColor::white(), PixelColor::black()),
    );

    text.update_string("0")?;

    Ok(text.into_enum())
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::transform2d::Transform2D;

    use crate::layers::count::CountLayer;

    #[test]
    fn it_update_count_not_panic() {
        let mut count = CountLayer::new(
            FrameBufferConfig::mock(),
            Transform2D::new(Vector2D::zeros(), Size::new(100, 100)),
        )
        .unwrap();
        count.write_count(100);
    }
}
