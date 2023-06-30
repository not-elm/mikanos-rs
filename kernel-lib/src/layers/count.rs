use alloc::format;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::{LayerFindable, MultipleLayer};
use crate::layers::text::TextLayer;

use super::text::colors::TextColors;

#[derive(Delegate)]
pub struct CountLayer {
    #[to(Transformable2D, LayerUpdatable)]
    layers: MultipleLayer,
}


impl CountLayer {
    #[inline]
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> KernelResult<Self> {
        Ok(Self {
            layers: count_layers(config, transform)?,
        })
    }


    #[inline]
    pub fn write_count(&mut self, count: usize) {
        self.layers
            .find_by_key_mut("count text")
            .unwrap()
            .require_text()
            .unwrap()
            .update_string(format!("{}", count).as_str())
            .unwrap();
    }


    #[inline]
    pub fn into_enum(self) -> Layer {
        Layer::Count(self)
    }
}


fn count_layers(config: FrameBufferConfig, transform: Transform2D) -> KernelResult<MultipleLayer> {
    let mut layers = MultipleLayer::new(transform);

    layers.new_layer(text_layer(config, &layers.transform())?);

    Ok(layers)
}


fn text_layer(config: FrameBufferConfig, _root_transform: &Transform2D) -> KernelResult<LayerKey> {
    let pos = Vector2D::zeros();

    let mut text = TextLayer::new(
        config,
        pos,
        Size::new(10, 1),
        TextColors::default()
            .change_foreground(PixelColor::black())
            .change_background(PixelColor::window_background()),
        false,
        None
    ).unwrap();

    text.update_string("0")?;

    Ok(text
        .into_enum()
        .into_layer_key("count text"))
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
