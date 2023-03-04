use crate::{gop::pixel::pixel_color::PixelColor, error::KernelResult};

#[warn(drop_bounds)]
pub trait PixelWritable {
    /// # Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult;
}
