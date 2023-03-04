use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
#[derive(Default)]
pub struct GbrPixelWriter {}

impl GbrPixelWriter {
    pub fn new() -> Self {
        Self {}
    }
}
impl PixelWritable for GbrPixelWriter {
    unsafe fn write(
        &mut self,
        frame_buffer_ptr: &mut *mut u8,
        pixel_pos: usize,
        color: &PixelColor,
    ) {
        let write_base_ptr = frame_buffer_ptr.offset((pixel_pos) as isize);
        write_base_ptr.write(color.b());
        write_base_ptr.offset(1).write(color.g());
        write_base_ptr.offset(2).write(color.r());
    }
}
