use auto_delegate::Delegate;

use common_lib::frame_buffer::PixelFormat;

use crate::gop::pixel::mapper::bgr_pixel_mapper::BgrPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::mapper::rgb_pixel_mapper::RgbPixelMapper;

#[derive(Debug, Clone, Delegate)]
#[to(PixelMapper)]
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
