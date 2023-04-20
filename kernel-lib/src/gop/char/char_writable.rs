use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub trait CharWritable {
    /// write char on the display using a frame buffer
    fn write(
        &mut self,
        c: char,
        pos: Vector2D<usize>,
        color: &PixelColor,
        pixel_writer: &mut impl PixelWritable,
    ) -> KernelResult;
}
