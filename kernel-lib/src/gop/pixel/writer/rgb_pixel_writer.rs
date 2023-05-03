use common_lib::frame_buffer::FrameBufferConfig;

use crate::error::KernelResult;
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::writer::pixel_writable::{flush_frame_buff, PixelFlushable, PixelWritable};

#[derive(Clone, Debug)]
pub struct RgbPixelWriter {
    frame_buffer_ptr: *mut u8,
    frame_buffer_config: FrameBufferConfig,
}

impl RgbPixelWriter {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Self {
        let frame_buffer_ptr = frame_buffer_config.frame_buffer_base_ptr();
        Self {
            frame_buffer_ptr,
            frame_buffer_config,
        }
    }


    fn write_buff(&self, buff: &mut [u8], color: &PixelColor) {
        buff[0] = color.r();
        buff[1] = color.g();
        buff[2] = color.b();
    }
}

impl Drop for RgbPixelWriter {
    fn drop(&mut self) {
        // unsafe {
        //     core::ptr::drop_in_place(self.frame_buffer_ptr);
        // };
    }
}

impl PixelWritable for RgbPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        let pixel_pos = calc_pixel_pos(&self.frame_buffer_config, x, y)?;
        let buff = core::slice::from_raw_parts_mut(
            self.frame_buffer_ptr
                .add(pixel_pos),
            4,
        );

        self.write_buff(buff, color);

        Ok(())
    }


    unsafe fn write_shadow_buff(
        &mut self,
        buff: &mut [u8],
        x: usize,
        y: usize,
        color: &PixelColor,
    ) -> KernelResult {
        let pixel_pos = calc_pixel_pos(&self.frame_buffer_config, x, y)?;
        self.write_buff(&mut buff[pixel_pos..pixel_pos + 4], color);
        Ok(())
    }
}


impl PixelFlushable for RgbPixelWriter {
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
        flush_frame_buff(pixel_frame, &self.frame_buffer_config)
    }
}
