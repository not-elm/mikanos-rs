use core::any::Any;

use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::layers::drawer::rect_colors::RectColors;
use crate::layers::drawer::LayerDrawable;

#[derive(Debug, Clone)]
pub struct ShapeDrawer {
    colors: RectColors,
}


impl ShapeDrawer {
    pub const fn new(colors: RectColors) -> Self {
        Self { colors }
    }
}


impl LayerDrawable for ShapeDrawer {
    fn draw_in_area(
        &mut self,
        pixels: &mut [u8],
        pixel_writer: &mut FrameBufferPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for pos in draw_area.points() {
            unsafe {
                pixel_writer.write(pixels, &pos, &self.colors.foreground())?;
            }
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
