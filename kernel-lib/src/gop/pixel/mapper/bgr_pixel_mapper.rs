use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct BgrPixelMapper;


impl BgrPixelMapper {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }
}


impl Default for BgrPixelMapper {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl PixelMapper for BgrPixelMapper {
    fn pixel_len(&self) -> usize {
        4
    }


    #[inline(always)]
    fn convert_to_buff(&mut self, color: &PixelColor) -> [u8; 4] {
        [
            color.b(),
            color.g(),
            color.r(),
            0,
        ]
    }
}


#[cfg(test)]
mod tests {
    use common_lib::array::array_eq;

    use crate::gop::pixel::mapper::bgr_pixel_mapper::BgrPixelMapper;
    use crate::gop::pixel::mapper::PixelMapper;
    use crate::gop::pixel::pixel_color::PixelColor;

    #[test]
    fn it_correct_bgr_buff() {
        let mut con = BgrPixelMapper::new();
        let buff = con.convert_to_buff(&PixelColor::blue());
        assert!(array_eq(&buff, &[0xFF, 0, 0, 0]));
    }
}
