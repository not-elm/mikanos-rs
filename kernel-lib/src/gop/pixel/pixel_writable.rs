use crate::gop::pixel::pixel_color::PixelColor;

#[warn(drop_bounds)]
pub trait PixelWritable {
    /// # Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    unsafe fn write(&mut self, pixel_pos: usize, color: &PixelColor);
}
