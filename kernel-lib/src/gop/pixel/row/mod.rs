use alloc::vec::Vec;

use common_lib::math::vector::Vector2D;

use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::pixel_converter::PixelConvertable;
use crate::gop::pixel::Pixel;

pub mod bgr_pixel_converter;
pub mod pixel_converter;
pub mod rgb_pixel_converter;

/// フレームバッファの一行分のピクセルを表します。
#[derive(Debug, Clone)]
pub struct PixelRow<Convert> {
    row: Vec<Pixel>,
    converter: Convert,
    pixels_buff: Vec<u8>,
}


impl<Convert: PixelConvertable> PixelRow<Convert> {
    pub fn new(
        row: Vec<Pixel>,
        mut converter: Convert,
        transparent_color: Option<PixelColor>,
    ) -> Self {
        let pixels_buff = concat_all(
            &row,
            &mut converter,
            transparent_color.unwrap_or(DISPLAY_BACKGROUND_COLOR),
        );
        Self {
            row,
            converter,
            pixels_buff,
        }
    }


    pub fn origin_pos(&self) -> Vector2D<usize> {
        self.row[0].pos
    }


    pub fn pixels_len_per_row(&self) -> usize {
        self.row.len() * self.converter.pixel_len()
    }


    pub fn pixels_buff(&mut self) -> &[u8] {
        &self.pixels_buff
    }
}


fn concat_all(
    row: &Vec<Pixel>,
    converter: &mut impl PixelConvertable,
    transparent_color: PixelColor,
) -> Vec<u8> {
    let mut pixels_buff: Vec<u8> = Vec::with_capacity(row.len() * converter.pixel_len());

    row.iter().for_each(|pixel| {
        let buff = converter.convert_to_buff(
            &pixel
                .color
                .unwrap_or(transparent_color),
        );


        pixels_buff.extend(buff);
    });


    pixels_buff
}


#[cfg(test)]
mod tests {
    use alloc::vec;

    use common_lib::array::eq_array;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::row::rgb_pixel_converter::RgbPixelConverter;
    use crate::gop::pixel::row::PixelRow;
    use crate::gop::pixel::Pixel;
    use crate::layers::window::drawers::cursor::cursor_buffer::{CursorBuffer, CURSOR_WIDTH};

    #[test]
    fn it_correct_length() {
        let row = PixelRow::new(
            vec![
                Pixel::default(),
                Pixel::default(),
            ],
            RgbPixelConverter::default(),
            None,
        );


        assert_eq!(row.pixels_len_per_row(), 8);
    }


    #[test]
    fn it_convert_to_row_pixels_buff() {
        let cursor_buff = CursorBuffer::default();
        let pixel_frame =
            cursor_buff.pixel_frame(Vector2D::zeros(), PixelColor::white(), PixelColor::yellow());

        let mut rows = pixel_frame.into_pixels(RgbPixelConverter::new(), Some(PixelColor::black()));


        let row = rows.get_mut(0).unwrap();


        let mut expect = [0; CURSOR_WIDTH * 4];
        expect[0] = 0xFF;
        expect[1] = 0xFF;

        assert_eq!(row.pixels_len_per_row(), expect.len());
        let row_pixels_buff = row.pixels_buff();
        assert!(eq_array(row_pixels_buff, &expect));
    }
}
