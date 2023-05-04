use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::shape_colors::ShapeColors;

#[derive(Debug, Clone)]
pub struct ShapeDrawer {
    colors: ShapeColors,
    pixel_writer: FrameBufferPixelWriter,
}


impl ShapeDrawer {
    pub const fn new(config: FrameBufferConfig, colors: ShapeColors) -> Self {
        Self {
            colors,
            pixel_writer: FrameBufferPixelWriter::new(config),
        }
    }
}


impl LayerUpdatable for ShapeDrawer {
    fn update_shadow_buffer(
        &mut self,
        shadow_frame: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        self.pixel_writer
            .fill_rect(shadow_frame, draw_area, self.colors.foreground_ref())
    }
}
