use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::pixel_converter::PixelConvertable;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct BgrPixelConverter([u8; 4]);


impl BgrPixelConverter {
    pub const fn new() -> Self {
        Self([0, 0, 0, 0])
    }
}


impl Default for BgrPixelConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PixelConvertable for BgrPixelConverter {
    fn pixel_len(&self) -> usize {
        4
    }

    fn convert_to_buff(&mut self, color: &PixelColor) -> &[u8] {
        self.0 = [
            color.b(),
            color.g(),
            color.r(),
            0,
        ];

        &self.0
    }
}


#[cfg(test)]
mod tests {
    use common_lib::array::eq_array;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::row::bgr_pixel_converter::BgrPixelConverter;
    use crate::gop::pixel::row::pixel_converter::PixelConvertable;

    #[test]
    fn it_correct_bgr_buff() {
        let mut con = BgrPixelConverter::new();
        let buff = con.convert_to_buff(&PixelColor::blue());
        assert!(eq_array(buff, &[0xFF, 0, 0, 0]));
    }
}
