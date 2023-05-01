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
    Rgb(FrameBufferConfig),
    Bgr(FrameBufferConfig),
}


impl EnumPixelWriter {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Self {
        match frame_buffer_config.pixel_format {
            PixelFormat::Rgb => Self::Rgb(frame_buffer_config),
            PixelFormat::Bgr => Self::Bgr(frame_buffer_config),
        }
    }
}


impl PixelWritable for EnumPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        match self {
            Self::Rgb(conf) => write_pixel(RgbPixelWriter::new(*conf), x, y, color),

            Self::Bgr(conf) => write_pixel(BgrPixelWriter::new(*conf), x, y, color),
        }
    }
}


impl PixelFlushable for EnumPixelWriter {
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
        match self {
            Self::Rgb(conf) => flush(RgbPixelWriter::new(*conf), pixel_frame),
            Self::Bgr(conf) => flush(BgrPixelWriter::new(*conf), pixel_frame),
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


unsafe fn flush(mut w: impl PixelFlushable, pixel_frame: PixelFrame) -> KernelResult {
    let result = w.flush(pixel_frame);
    // Do not free memory of a frame buffer
    core::mem::forget(w);
    result
}
