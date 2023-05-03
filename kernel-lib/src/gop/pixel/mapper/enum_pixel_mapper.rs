use common_lib::frame_buffer::PixelFormat;

use crate::gop::pixel::mapper::bgr_pixel_mapper::BgrPixelMapper;
use crate::gop::pixel::mapper::rgb_pixel_mapper::RgbPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone)]
pub enum EnumPixelMapper {
    Rgb(RgbPixelMapper),
    Bgr(BgrPixelMapper),
}


impl EnumPixelMapper {
    pub const fn new(pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::Rgb => Self::Rgb(RgbPixelMapper::new()),
            PixelFormat::Bgr => Self::Bgr(BgrPixelMapper::new()),
        }
    }
}


impl PixelMapper for EnumPixelMapper {
    fn pixel_len(&self) -> usize {
        match self {
            Self::Rgb(rbg) => rbg.pixel_len(),
            Self::Bgr(bgr) => bgr.pixel_len(),
        }
    }

    fn convert_to_buff(&mut self, color: &PixelColor) -> &[u8] {
        match self {
            Self::Rgb(rgb) => rgb.convert_to_buff(color),
            Self::Bgr(bgr) => bgr.convert_to_buff(color),
        }
    }
}
