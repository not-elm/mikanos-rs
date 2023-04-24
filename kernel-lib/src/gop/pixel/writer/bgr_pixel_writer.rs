use common_lib::frame_buffer::FrameBufferConfig;

use crate::error::KernelResult;
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::row::bgr_pixel_converter::BgrPixelConverter;
use crate::gop::pixel::writer::pixel_writable::{flush_frame_buff, PixelFlushable, PixelWritable};

pub struct BgrPixelWriter {
    frame_buffer_ptr: *mut u8,
    frame_buffer_config: FrameBufferConfig,
}


impl BgrPixelWriter {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Self {
        let frame_buffer_ptr = frame_buffer_config.frame_buffer_base_ptr();
        Self {
            frame_buffer_ptr,
            frame_buffer_config,
        }
    }
}


impl Drop for BgrPixelWriter {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.frame_buffer_ptr);
        };
    }
}

impl PixelWritable for BgrPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        let pixel_pos = calc_pixel_pos(&self.frame_buffer_config, x, y)?;
        let write_base_ptr = self
            .frame_buffer_ptr
            .add(pixel_pos);
        write_base_ptr.write(color.b());
        write_base_ptr
            .add(1)
            .write(color.g());
        write_base_ptr
            .add(2)
            .write(color.r());
        Ok(())
    }
}


impl PixelFlushable for BgrPixelWriter {
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
        flush_frame_buff(
            pixel_frame,
            &self.frame_buffer_config,
            BgrPixelConverter::default(),
        )
    }
}
