use auto_delegate::Delegate;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::transform2d::Transform2D;

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::shape_drawer::ShapeDrawer;

pub mod shape_colors;
pub mod shape_drawer;


#[derive(Debug, Delegate)]
pub struct ShapeLayer {
    drawer: ShapeDrawer,

    #[to(Transformable2D)]
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
    fn update_back_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        self.drawer
            .update_back_buffer(shadow_frame_buff, draw_area)
    }
}
