use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub struct RgbPixelWriter(*mut u8);

impl RgbPixelWriter {
    pub fn new(frame_buffer_base_addr: u64) -> Self {
        Self(frame_buffer_base_addr as *mut u8)
    }
}

impl Drop for RgbPixelWriter {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.0);
        };
    }
}

impl PixelWritable for RgbPixelWriter {
    unsafe fn write(&mut self, pixel_pos: usize, color: &PixelColor) {
        let write_base_ptr = self.0.offset((pixel_pos) as isize);
        write_base_ptr.write(color.r());
        write_base_ptr.offset(1).write(color.g());
        write_base_ptr.offset(2).write(color.b());
    }
}
