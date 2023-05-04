use common_lib::impl_transformable2D;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::transform2d::Transform2D;

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::shape_drawer::ShapeDrawer;

pub mod shape_colors;
pub mod shape_drawer;


#[derive(Debug)]
pub struct ShapeLayer {
    drawer: ShapeDrawer,
    transform: Transform2D,
}


impl ShapeLayer {
    pub const fn new(drawer: ShapeDrawer, transform: Transform2D) -> Self {
        Self { drawer, transform }
    }


    pub fn into_enum(self) -> Layer {
        Layer::Shape(self)
    }
}


impl LayerUpdatable for ShapeLayer {
    fn update_shadow_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        self.drawer
            .update_shadow_buffer(shadow_frame_buff, draw_area)
    }
}


impl_transformable2D!(ShapeLayer);
