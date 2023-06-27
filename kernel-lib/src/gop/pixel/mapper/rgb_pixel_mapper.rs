use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RgbPixelMapper;


impl RgbPixelMapper {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }
}


impl Default for RgbPixelMapper {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}


impl PixelMapper for RgbPixelMapper {
    #[inline(always)]
    fn pixel_len(&self) -> usize {
        4
    }


    #[inline(always)]
    fn convert_to_buff(&mut self, color: &PixelColor) -> [u8; 4] {
        [
            color.r(),
            color.g(),
            color.b(),
            0,
        ]
    }
}


#[cfg(test)]
mod tests {
    use common_lib::array::array_eq;

    use crate::gop::pixel::mapper::PixelMapper;
    use crate::gop::pixel::mapper::rgb_pixel_mapper::RgbPixelMapper;
    use crate::gop::pixel::pixel_color::PixelColor;

    #[test]
    fn it_correct_buff() {
        let mut con = RgbPixelMapper::new();
        let buff = con.convert_to_buff(&PixelColor::blue());
        assert!(array_eq(&buff, &[0, 0, 0xFF, 0]));
    }
}
