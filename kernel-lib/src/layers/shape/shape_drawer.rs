use core::cell::Cell;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer_updatable::LayerUpdatable;

#[derive(Debug, Clone)]
pub struct ShapeDrawer {
    color: Cell<PixelColor>,
    pixel_writer: FrameBufferPixelWriter,
}


impl ShapeDrawer {
    #[inline(always)]
    pub const fn new(config: FrameBufferConfig, color: PixelColor) -> Self {
        Self {
            color: Cell::new(color),
            pixel_writer: FrameBufferPixelWriter::new(config),
        }
    }


    #[inline]
    pub fn set_color(&self, color: PixelColor) {
        self.color.replace(color);
    }
}


impl LayerUpdatable for ShapeDrawer {
    #[inline(always)]
    fn update_back_buffer(
        &mut self,
        shadow_frame: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        self.pixel_writer
            .fill_rect(shadow_frame, draw_area, &self.color.get())
    }
}
