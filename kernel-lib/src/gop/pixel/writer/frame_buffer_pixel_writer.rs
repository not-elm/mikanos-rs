use alloc::vec::Vec;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::calc_pixel_pos_from_vec2d;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;

#[derive(Debug, Clone)]
pub struct FrameBufferPixelWriter {
    config: FrameBufferConfig,
    mapper: EnumPixelMapper,
}


impl FrameBufferPixelWriter {
    pub const fn new(config: FrameBufferConfig) -> Self {
        let mapper = EnumPixelMapper::new(config.pixel_format);
        Self { config, mapper }
    }


    pub fn fill_rect(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
        color: &PixelColor,
    ) -> KernelResult {
        let mut v1: Vec<u8> = Vec::new();
        for _ in 0..draw_area.size().width() {
            v1.extend_from_slice(
                &self.mapper
                    .convert_to_buff(color),
            );
        }

        let v = v1.as_slice();

        for y in draw_area.origin().y()..draw_area.end().y() {
            let i = calc_pixel_pos_from_vec2d(
                shadow_buff.config_ref(),
                &Vector2D::new(draw_area.origin().x(), y),
            )?;

            shadow_buff.raw_mut()[i..i + v.len()].copy_from_slice(v);
        }

        Ok(())
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
