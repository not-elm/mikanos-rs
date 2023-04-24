use crate::gop::pixel::pixel_color::PixelColor;

pub trait PixelConvertable {
    /// 1ピクセルあたりのバイト数
    fn pixel_len(&self) -> usize;


    fn convert_to_buff(&mut self, color: &PixelColor) -> &[u8];
}
