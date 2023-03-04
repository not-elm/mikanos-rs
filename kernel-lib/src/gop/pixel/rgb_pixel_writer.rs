use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

#[derive(Default)]
pub struct RgbPixelWriter {}

impl RgbPixelWriter {
    pub fn new() -> Self {
        Self {}
    }
}

impl PixelWritable for RgbPixelWriter {
    unsafe fn write(
        &mut self,
        frame_buffer_ptr: &mut *mut u8,
        pixel_pos: usize,
        color: &PixelColor,
    ) {
        let write_base_ptr = frame_buffer_ptr.offset((pixel_pos) as isize);
        write_base_ptr.write(color.r());
        write_base_ptr.offset(1).write(color.g());
        write_base_ptr.offset(2).write(color.b());
    }
}
