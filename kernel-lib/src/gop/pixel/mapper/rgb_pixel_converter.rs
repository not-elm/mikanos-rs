use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RgbPixelConverter([u8; 4]);


impl RgbPixelConverter {
    pub const fn new() -> Self {
        Self([0, 0, 0, 0])
    }
}


impl Default for RgbPixelConverter {
    fn default() -> Self {
        Self::new()
    }
}


impl PixelMapper for RgbPixelConverter {
    fn pixel_len(&self) -> usize {
        4
    }

    fn convert_to_buff(&mut self, color: &PixelColor) -> &[u8] {
        self.0 = [
            color.r(),
            color.g(),
            color.b(),
            0,
        ];

        &self.0
    }
}


#[cfg(test)]
mod tests {
    use common_lib::array::eq_array;

    use crate::gop::pixel::mapper::rgb_pixel_converter::RgbPixelConverter;
    use crate::gop::pixel::mapper::PixelMapper;
    use crate::gop::pixel::pixel_color::PixelColor;

    #[test]
    fn it_correct_buff() {
        let mut con = RgbPixelConverter::new();
        let buff = con.convert_to_buff(&PixelColor::blue());
        assert!(eq_array(buff, &[0, 0, 0xFF, 0]));
    }
}
