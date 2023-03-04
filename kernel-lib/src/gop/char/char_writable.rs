use common_lib::vector::Vector2D;

use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub trait CharWritable {
    /// write char on the display using a frame buffer
    fn write(
        &mut self,
        c: char,
        pos: Vector2D,
        color: &PixelColor,
        pixel_writer: &mut impl PixelWritable,
    );
}
