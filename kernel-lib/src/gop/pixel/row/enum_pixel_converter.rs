use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::bgr_pixel_converter::BgrPixelConverter;
use crate::gop::pixel::row::pixel_converter::PixelConvertable;
use crate::gop::pixel::row::rgb_pixel_converter::RgbPixelConverter;
use common_lib::frame_buffer::PixelFormat;


#[derive(Debug, Clone)]
pub enum EnumPixelConverter {
    Rgb(RgbPixelConverter),
    Bgr(BgrPixelConverter),
}


impl EnumPixelConverter {
    pub fn new(pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::Rgb => Self::Rgb(RgbPixelConverter::new()),
            PixelFormat::Bgr => Self::Bgr(BgrPixelConverter::new()),
        }
    }
}


impl PixelConvertable for EnumPixelConverter {
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
