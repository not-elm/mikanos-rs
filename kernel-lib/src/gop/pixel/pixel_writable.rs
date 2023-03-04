use common_lib::frame_buffer::PixelFormat;

use crate::gop::pixel::gbr_pixel_writer::GbrPixelWriter;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::rgb_pixel_writer::RgbPixelWriter;

pub trait PixelWritable {
    /// # Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    unsafe fn write(
        &mut self,
        frame_buffer_ptr: &mut *mut u8,
        pixel_pos: usize,
        color: &PixelColor,
    );
}

impl PixelWritable for PixelFormat {
    unsafe fn write(
        &mut self,
        frame_buffer_ptr: &mut *mut u8,
        pixel_pos: usize,
        color: &PixelColor,
    ) {
        match self {
            Self::Rgb => RgbPixelWriter::new().write(frame_buffer_ptr, pixel_pos, color),
            Self::Bgr => GbrPixelWriter::new().write(frame_buffer_ptr, pixel_pos, color),
        }
    }
}
