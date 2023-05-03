use core::any::Any;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::gop::pixel::writer::enum_pixel_writer::EnumPixelWriter;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::layers::drawer::rect_colors::RectColors;
use crate::layers::drawer::LayerDrawable;

#[derive(Debug, Clone)]
pub struct ShapeDrawer {
    colors: RectColors,
    converter: EnumPixelConverter,
}


impl ShapeDrawer {
    pub fn new(colors: RectColors, pixel_format: PixelFormat) -> Self {
        Self {
            colors,
            converter: EnumPixelConverter::new(pixel_format),
        }
    }
}


impl LayerDrawable for ShapeDrawer {
    fn draw_in_area(
        &mut self,
        pixels: &mut [u8],
        pixel_writer: &mut EnumPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for pos in draw_area.points() {
            unsafe {
                pixel_writer.write_shadow_buff(
                    pixels,
                    pos.x(),
                    pos.y(),
                    &self.colors.foreground(),
                )?;
            }
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
