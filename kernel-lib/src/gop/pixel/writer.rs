use crate::gop::pixel::color::Color;
use common_lib::frame_buffer::FrameBufferConfig;

pub trait PixelWritable {
    unsafe fn write(&mut self, frame_buffer_ptr: &mut *mut u8, pixel_pos: usize, color: &Color);
}

pub struct RgbPixelWriter {}

impl PixelWritable for RgbPixelWriter {
    unsafe fn write(&mut self, frame_buffer_ptr: &mut *mut u8, pixel_pos: usize, color: &Color) {
        let write_base_ptr = frame_buffer_ptr.offset((pixel_pos) as isize);
        write_base_ptr.write(color.r());
        // write_base_ptr.offset(1).write(color.g());
        // write_base_ptr.offset(2).write(color.b());
    }
}

pub unsafe fn write_pixel(
    writer: &mut impl PixelWritable,
    frame_buffer_config: &FrameBufferConfig,
    x: i32,
    y: i32,
    color: &Color,
) {
    let pixel_pos = 4 * (frame_buffer_config.pixel_per_scanline * x as usize + y as usize);

    let mut frame_buffer_ptr = frame_buffer_config.frame_buffer_base_ptr();

    writer.write(&mut frame_buffer_ptr, pixel_pos, color);
}
