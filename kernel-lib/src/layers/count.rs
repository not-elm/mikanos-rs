use alloc::format;

use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;

use crate::error::KernelResult;
use crate::gop;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::{LayerFindable, MultipleLayer};
use crate::layers::text::{config, TextLayer};

#[derive(Delegate)]
pub struct CountLayer {
    #[to(Transformable2D, LayerUpdatable)]
    layers: MultipleLayer,
}


impl CountLayer {
    #[inline]
    pub fn new(transform: Transform2D) -> KernelResult<Self> {
        Ok(Self {
            layers: count_layers(transform)?,
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


fn count_layers(transform: Transform2D) -> KernelResult<MultipleLayer> {
    let mut layers = MultipleLayer::new(transform);

    layers.new_layer(text_layer()?);

    Ok(layers)
}


fn text_layer() -> KernelResult<LayerKey> {
    let pos = Vector2D::zeros();
    let config = config::Builder::new()
        .foreground(PixelColor::black())
        .background(PixelColor::window_background())
        .build();

    let mut text = TextLayer::new(gop::config(), pos, Size::new(10, 1), config).unwrap();

    text.update_string("0")?;

    Ok(text
        .into_enum()
        .into_layer_key("count text"))
}


#[cfg(test)]
mod tests {
    use crate::gop;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::transform2d::Transform2D;

    use crate::layers::count::CountLayer;

    #[test]
    fn it_update_count_not_panic() {
        gop::test_init();
        let mut count =
            CountLayer::new(Transform2D::new(Vector2D::zeros(), Size::new(100, 100))).unwrap();
        count.write_count(100);
    }
}
