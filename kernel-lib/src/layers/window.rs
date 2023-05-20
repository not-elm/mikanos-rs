use alloc::vec;
use alloc::vec::Vec;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::shape_colors::ShapeColors;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;

#[derive(Delegate)]
pub struct WindowLayer {
    layers: Vec<Layer>,

    #[to(Transformable2D)]
    transform: Transform2D,
}


impl WindowLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D) -> Self {
        let toolbar = ShapeLayer::new(
            ShapeDrawer::new(config, ShapeColors::new(PixelColor::white(), None)),
            Transform2D::new(Vector2D::zeros(), Size::new(transform.size().width(), 100)),
        )
        .into_enum();

        Self {
            layers: vec![toolbar],
            transform,
        }
    }


    pub const fn into_enum(self) -> Layer {
        Layer::Window(self)
    }
}


impl LayerUpdatable for WindowLayer {
    fn update_shadow_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for layer in self.layers.iter_mut() {
            layer.update_shadow_buffer(shadow_frame_buff, draw_area)?;
        }

        Ok(())
    }
}
