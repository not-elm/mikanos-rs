use crate::gop::pixel::pixel_color::PixelColor;

pub mod bgr_pixel_converter;
pub mod enum_pixel_converter;
pub mod rgb_pixel_converter;


pub trait PixelMapper {
    /// 1ピクセルあたりのバイト数
    fn pixel_len(&self) -> usize;


    fn convert_to_buff(&mut self, color: &PixelColor) -> &[u8];
}
