use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::writer::bgr_pixel_writer::BgrPixelWriter;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
use crate::gop::pixel::writer::rgb_pixel_writer::RgbPixelWriter;

#[derive(Clone, Debug)]
#[cfg_attr(test, allow(dead_code))]
pub enum EnumPixelWriter {
    Rgb(RgbPixelWriter),
    Bgr(BgrPixelWriter),
}


impl EnumPixelWriter {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Self {
        match frame_buffer_config.pixel_format {
            PixelFormat::Rgb => Self::Rgb(RgbPixelWriter::new(frame_buffer_config)),
            PixelFormat::Bgr => Self::Bgr(BgrPixelWriter::new(frame_buffer_config)),
        }
    }
}


impl PixelWritable for EnumPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        match self {
            Self::Rgb(write) => write.write(x, y, color),

            Self::Bgr(write) => write.write(x, y, color),
        }
    }

    unsafe fn write_shadow_buff(
        &mut self,
        buff: &mut [u8],
        x: usize,
        y: usize,
        color: &PixelColor,
    ) -> KernelResult {
        match self {
            Self::Rgb(write) => write.write_shadow_buff(buff, x, y, color),

            Self::Bgr(write) => write.write_shadow_buff(buff, x, y, color),
        }
    }
}


impl PixelFlushable for EnumPixelWriter {
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
        match self {
            Self::Rgb(writer) => writer.flush(pixel_frame),
            Self::Bgr(writer) => writer.flush(pixel_frame),
        }
    }
}
