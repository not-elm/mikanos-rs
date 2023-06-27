use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::layers::text::colors::TextColors;

pub trait CharWritable {
    /// write char on the display using a frame buffer
    fn write(
        &mut self,
        dist_buff: &mut [u8],
        c: char,
        pos: Vector2D<usize>,
        colors: &TextColors,
        pixel_writer: &mut impl PixelWritable,
    ) -> KernelResult;


    fn font_unit(&self) -> Size;
}
