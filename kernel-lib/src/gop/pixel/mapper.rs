use crate::error::KernelResult;
use crate::gop::pixel::calc_pixel_pos_from_vec2d;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;

pub mod bgr_pixel_mapper;
pub mod enum_pixel_mapper;
pub mod rgb_pixel_mapper;


pub trait PixelMapper {
    /// 1ピクセルあたりのバイト数
    fn pixel_len(&self) -> usize;


    fn convert_to_buff(&mut self, color: &PixelColor) -> &[u8];


    fn write_frame_buff(
        &mut self,
        config: &FrameBufferConfig,
        frame_buff: &mut [u8],
        pos: &Vector2D<usize>,
        color: &PixelColor,
    ) -> KernelResult {
        let origin = calc_pixel_pos_from_vec2d(config, pos)?;
        let end = origin + self.pixel_len();
        frame_buff[origin..end].copy_from_slice(self.convert_to_buff(color));

        Ok(())
    }
}
