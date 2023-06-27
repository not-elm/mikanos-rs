use alloc::vec::Vec;

use common_lib::math::vector::Vector2D;

use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::{calc_pixel_pos_from_vec2d, Pixel};
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;

/// フレームバッファの一行分のピクセルを表します。
#[derive(Debug, Clone)]
pub struct PixelRow<Convert> {
    row: Vec<Pixel>,
    converter: Convert,
}


impl<Convert: PixelMapper> PixelRow<Convert> {
    #[inline]
    pub fn new(row: Vec<Pixel>, converter: Convert) -> Self {
        Self { row, converter }
    }


    #[inline]
    pub fn origin_pos(&self) -> Vector2D<usize> {
        self.row[0].pos
    }


    #[inline]
    pub fn end_pos(&self) -> Vector2D<usize> {
        self.row[self.row.len() - 1].pos
    }


    #[inline]
    pub fn pixels_len_per_row(&self) -> usize {
        self.row.len() * self.converter.pixel_len()
    }


    #[inline]
    pub fn pixels_len(&self) -> usize {
        self.row.len()
    }


    #[inline]
    pub fn pixels_buff(mut self, back_buff: &ShadowFrameBuffer) -> Vec<u8> {
        concat_all(&self.row, &mut self.converter, back_buff)
    }


    #[inline]
    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.row
    }
}


fn concat_all(
    row: &Vec<Pixel>,
    converter: &mut impl PixelMapper,
    back_buff: &ShadowFrameBuffer,
) -> Vec<u8> {
    let mut pixels_buff: Vec<u8> = Vec::with_capacity(row.len() * converter.pixel_len());

    row.iter().for_each(|pixel| {
        pixels_buff.extend(buff(pixel, converter, back_buff));
    });


    pixels_buff
}


#[inline]
fn buff(pixel: &Pixel, converter: &mut impl PixelMapper, back_buff: &ShadowFrameBuffer) -> [u8; 4] {
    if let Some(color) = pixel.color {
        converter.convert_to_buff(&color)
    } else {
        let origin = calc_pixel_pos_from_vec2d(back_buff.config_ref(), &pixel.pos()).unwrap();
        let buff = back_buff.raw_ref();

        [
            buff[origin],
            buff[origin + 1],
            buff[origin + 2],
            buff[origin + 3],
        ]
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use common_lib::array::array_eq;
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
    use crate::gop::pixel::mapper::rgb_pixel_mapper::RgbPixelMapper;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::pixel_row::PixelRow;
    use crate::gop::pixel::Pixel;
    use crate::layers::cursor::buffer::{CursorBuffer, CURSOR_WIDTH};
    use crate::layers::cursor::colors::CursorColors;

    #[test]
    fn it_correct_length() {
        let row = PixelRow::new(
            vec![
                Pixel::default(),
                Pixel::default(),
            ],
            RgbPixelMapper::default(),
            None,
        );


        assert_eq!(row.pixels_len_per_row(), 8);
    }


    #[test]
    fn it_convert_to_row_pixels_buff() {
        let cursor_buff = CursorBuffer::default();
        let pixel_frame = cursor_buff.pixel_frame(
            Vector2D::zeros(),
            None,
            CursorColors::default()
                .change_border(PixelColor::yellow())
                .change_transparent(PixelColor::black()),
            EnumPixelMapper::new(PixelFormat::Rgb),
        );

        let mut rows: Vec<PixelRow<EnumPixelMapper>> = pixel_frame.collect();
        let row = rows.get_mut(0).unwrap();

        let mut expect = [0; CURSOR_WIDTH * 4];
        expect[0] = 0xFF;
        expect[1] = 0xFF;

        assert_eq!(row.pixels_len_per_row(), expect.len());
        let row_pixels_buff = row.pixels_buff();
        assert!(array_eq(row_pixels_buff, &expect));
    }
}
