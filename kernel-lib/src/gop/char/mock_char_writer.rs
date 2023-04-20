use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

/// For Test
#[derive(Default)]
pub struct MockCharWriter {}

impl MockCharWriter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl CharWritable for MockCharWriter {
    fn write(
        &mut self,
        _c: char,
        _pos: Vector2D<usize>,
        _color: &PixelColor,
        _pixel_writer: &mut impl PixelWritable,
    ) -> KernelResult {
        Ok(())
    }
}
