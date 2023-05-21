use alloc::vec::Vec;

use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;

pub struct MultipleLayer {
    layers: Vec<Layer>,

    transform: Transform2D,
}


impl MultipleLayer {
    pub const fn new(transform: Transform2D) -> Self {
        Self {
            layers: Vec::new(),
            transform,
        }
    }


    pub fn new_layer(&mut self, mut layer: Layer) {
        layer.move_to(layer.pos() + self.pos());
        self.layers.push(layer);
    }
}


impl Transformable2D for MultipleLayer {
    fn move_to(&mut self, pos: Vector2D<usize>) {
        self.transform.move_to(pos);

        let relative = pos.relative(self.pos());

        self.layers
            .iter_mut()
            .for_each(|layer| layer.move_to_relative(relative));
    }

    fn resize(&mut self, size: Size) {
        self.transform.resize(size)
    }

    fn rect(&self) -> Rectangle<usize> {
        self.transform.rect()
    }

    fn pos(&self) -> Vector2D<usize> {
        self.transform.pos()
    }

    fn transform_ref(&self) -> &Transform2D {
        self.transform.transform_ref()
    }
}


impl LayerUpdatable for MultipleLayer {
    fn update_shadow_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for layer in self.layers.iter_mut() {
            if let Some(draw_rect) = draw_area.intersect(&layer.rect()) {
                layer.update_shadow_buffer(shadow_frame_buff, &draw_area)?;
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

    use crate::layers::layer::Layer;
    use crate::layers::multiple_layer::MultipleLayer;
    use crate::layers::shape::shape_colors::ShapeColors;
    use crate::layers::shape::shape_drawer::ShapeDrawer;
    use crate::layers::shape::ShapeLayer;

    #[test]
    fn it_move_to_layer() {
        let mut layer = MultipleLayer::new(Transform2D::new(
            Vector2D::new(100, 100),
            Size::new(100, 100),
        ));

        layer.new_layer(Layer::Shape(ShapeLayer::new(
            ShapeDrawer::new(FrameBufferConfig::mock(), ShapeColors::default()),
            Transform2D::new(Vector2D::zeros(), Size::new(100, 100)),
        )));

        layer.move_to_relative(Vector2D::new(10, 10));

        assert_eq!(layer.layers[0].pos(), Vector2D::new(110, 110));
    }
}
