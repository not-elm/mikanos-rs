use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

use common_lib::vector::Vector2D;

pub trait CharWritable {
    /// write char on the display using a frame buffer
    fn write(
        &mut self,
        c: char,
        pos: Vector2D,
        frame_buffer_ptr: *mut u8,
        color: &PixelColor,
        pixel_writer: &mut impl PixelWritable,
    );
}
