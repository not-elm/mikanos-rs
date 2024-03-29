use alloc::vec::Vec;
use core::num::TryFromIntError;

use auto_delegate::delegate;

use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::layer_updatable::LayerUpdatable;

pub struct MultipleLayer {
    layers: Vec<LayerKey>,

    transform: Transform2D,
}


#[delegate]
pub trait LayerFindable {
    /// Gets the layer with the target key from the descendant layers.
    fn find_by_key(&self, key: &str) -> Option<&Layer>;


    /// Gets the layer with the target key from the descendant layers.
    fn find_by_key_mut(&mut self, key: &str) -> Option<&mut Layer>;
}


impl MultipleLayer {
    #[inline(always)]
    pub const fn new(transform: Transform2D) -> Self {
        Self {
            layers: Vec::new(),
            transform,
        }
    }


    pub fn new_layer(&mut self, mut layer: LayerKey) {
        layer.move_to(layer.pos() + self.pos());

        self.layers.push(layer);
    }
    
    
    pub fn then_add(mut self,  layer: LayerKey) -> Self {
        self.new_layer(layer);
        self
    }


    #[inline(always)]
    pub fn layers_mut(&mut self) -> &mut Vec<LayerKey> {
        &mut self.layers
    }


    #[inline(always)]
    pub fn into_enum(self) -> Layer {
        Layer::Multiple(self)
    }


    /// # Panics
    ///
    /// Panics if the layer associated with the key does not exist.
    #[inline]
    pub fn force_find_by_key_mut(&mut self, key: &str) -> &mut Layer {
        self.find_by_key_mut(key)
            .unwrap()
    }
}


impl LayerFindable for MultipleLayer {
    #[inline]
    fn find_by_key(&self, key: &str) -> Option<&Layer> {
        self.layers
            .iter()
            .find_map(|layer| layer.find_by_key(key))
    }


    #[inline]
    fn find_by_key_mut(&mut self, key: &str) -> Option<&mut Layer> {
        self.layers
            .iter_mut()
            .find_map(|layer| layer.find_by_key_mut(key))
    }
}


impl Transformable2D for MultipleLayer {
    fn move_to(&mut self, pos: Vector2D<usize>) {
        let relative = pos.relative(self.pos());

        self.move_to_relative(relative)
            .unwrap_or(());
    }


    #[inline(always)]
    fn resize(&mut self, size: Size) {
        self.transform.resize(size)
    }


    #[inline(always)]
    fn rect(&self) -> Rectangle<usize> {
        self.transform.rect()
    }


    #[inline(always)]
    fn pos(&self) -> Vector2D<usize> {
        self.transform.pos()
    }


    #[inline(always)]
    fn transform(&self) -> Transform2D {
        self.transform.transform()
    }


    fn move_to_relative(&mut self, pos: Vector2D<isize>) -> Result<(), TryFromIntError> {
        if self
            .transform
            .move_to_relative(pos)
            .is_ok()
        {
            for layer in self.layers.iter_mut() {
                layer.move_to_relative(pos)?;
            }
        }

        Ok(())
    }
}


impl LayerUpdatable for MultipleLayer {
    fn update_back_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for layer in self.layers.iter_mut() {
            if let Some(draw_rect) = draw_area.intersect(&layer.rect()) {
                layer.update_back_buffer_in_area(shadow_frame_buff, &draw_rect)?;
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::transform2d::{Transform2D, Transformable2D};

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::layers::layer::Layer;
    use crate::layers::multiple_layer::MultipleLayer;
    use crate::layers::shape::shape_drawer::ShapeDrawer;
    use crate::layers::shape::ShapeLayer;

    #[test]
    fn it_move_to_layer() {
        let mut layer = MultipleLayer::new(Transform2D::new(
            Vector2D::new(100, 100),
            Size::new(100, 100),
        ));

        layer.new_layer(
            Layer::Shape(ShapeLayer::new(
                ShapeDrawer::new(FrameBufferConfig::mock(), PixelColor::black()),
                Transform2D::new(Vector2D::zeros(), Size::new(100, 100)),
            ))
                .into_layer_key(""),
        );

        layer
            .move_to_relative(Vector2D::new(10, 10))
            .unwrap();

        assert_eq!(layer.layers[0].pos(), Vector2D::new(110, 110));
    }
}
