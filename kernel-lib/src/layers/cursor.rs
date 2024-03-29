use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::cursor::colors::CursorColors;
use crate::layers::cursor::drawer::CursorDrawer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;

pub mod buffer;
pub mod colors;
pub mod cursor_pixel_iter;
pub mod drawer;

#[derive(Delegate)]
pub struct CursorLayer {
    drawer: CursorDrawer,

    #[to(Transformable2D)]
    transform: Transform2D,
}


impl CursorLayer {
    pub fn new(
        config: FrameBufferConfig,
        scale: Vector2D<usize>,
        cursor_colors: CursorColors,
    ) -> Self {
        let drawer = CursorDrawer::new(config, scale, cursor_colors);
        let transform = Transform2D::new(Vector2D::zeros(), drawer.cursor_size());
        Self { drawer, transform }
    }


    #[inline(always)]
    pub fn new_use_default(config: FrameBufferConfig) -> Self {
        Self::new(config, Vector2D::unit(), CursorColors::default())
    }


    #[inline(always)]
    pub fn into_enum(self) -> Layer {
        Layer::Cursor(self)
    }


    #[inline(always)]
    pub fn set_color(&mut self, colors: CursorColors) {
        self.drawer.set_color(colors);
    }
}


impl LayerUpdatable for CursorLayer {
    #[inline(always)]
    fn update_back_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        self.drawer
            .update_back_buffer(shadow_frame_buff, draw_area, self.transform.pos())?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::transform2d::Transformable2D;

    use crate::layers::cursor::colors::CursorColors;
    use crate::layers::cursor::CursorLayer;

    #[test]
    fn it_moved_to_unit() {
        let mut layer = CursorLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::unit(),
            CursorColors::default(),
        );
        layer.move_to(Vector2D::unit());

        assert_eq!(layer.transform.pos(), Vector2D::unit());
    }
}
