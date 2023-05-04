use common_lib::math::vector::Vector2D;

use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::{error::KernelResult, gop::pixel::pixel_color::PixelColor};

#[warn(drop_bounds)]
pub trait PixelWritable {
    /// # Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    unsafe fn write(
        &mut self,
        buff: &mut [u8],
        pos: &Vector2D<usize>,
        color: &PixelColor,
    ) -> KernelResult;
}


pub trait PixelFlushable {
    ///# Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    ///
    /// 描画域を表す構造体を元に、ピクセルを描画します。
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult;
}
