use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;

#[derive(Clone)]
pub struct FrameBufferPixelWriter {
    config: FrameBufferConfig,
    mapper: EnumPixelMapper,
}


impl FrameBufferPixelWriter {
    pub const fn new(config: FrameBufferConfig) -> Self {
        let mapper = EnumPixelMapper::new(config.pixel_format);
        Self { config, mapper }
    }
}


impl PixelWritable for FrameBufferPixelWriter {
    unsafe fn write(
        &mut self,
        frame_buff: &mut [u8],
        pos: &Vector2D<usize>,
        color: &PixelColor,
    ) -> KernelResult {
        self.mapper
            .write_frame_buff(&self.config, frame_buff, pos, color)
    }
}
