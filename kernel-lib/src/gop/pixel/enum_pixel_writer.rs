use crate::error::KernelResult;
use crate::gop::pixel::bgr_pixel_writer::BgrPixelWriter;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::gop::pixel::rgb_pixel_writer::RgbPixelWriter;
use common_lib::frame_buffer::FrameBufferConfig;

#[derive(Clone, Debug)]
pub enum EnumPixelWriter {
    Rgb(FrameBufferConfig),
    Bgr(FrameBufferConfig),
}
impl PixelWritable for EnumPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        match self {
            Self::Rgb(conf) => write_pixel(RgbPixelWriter::new(*conf), x, y, color),

            Self::Bgr(conf) => write_pixel(BgrPixelWriter::new(*conf), x, y, color),
        }
    }
}
impl Drop for EnumPixelWriter {
    fn drop(&mut self) {
        match self {
            Self::Rgb(conf) => {
                unsafe {
                    core::ptr::drop_in_place(conf.frame_buffer_base_ptr());
                };
            }
            Self::Bgr(conf) => {
                unsafe {
                    core::ptr::drop_in_place(conf.frame_buffer_base_ptr());
                };
            }
        }
    }
}
unsafe fn write_pixel(
    mut w: impl PixelWritable,
    x: usize,
    y: usize,
    color: &PixelColor,
) -> KernelResult {
    let result = w.write(x, y, color);
    // Do not free memory of a frame buffer
    core::mem::forget(w);
    result
}
