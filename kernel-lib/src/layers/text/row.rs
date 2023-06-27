use alloc::vec;
use alloc::vec::Vec;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::buff_pixel_writer::BuffPixelWriter;
use crate::layers::text::colors::TextColors;

pub struct TextRow {
    text_buffs: Vec<u8>,
    buff_pixel_writer: BuffPixelWriter,
    font_unit: Size,
    texts: Vec<char>,
    max_text_len: usize,
    background: PixelColor,
}


impl TextRow {
    pub fn new(
        background: PixelColor,
        font_unit: Size,
        max_text_len: usize,
        pixel_format: PixelFormat,
    ) -> Self {
        Self::new_with_buff(
            new_text_row_buff(&background, &font_unit, max_text_len, pixel_format),
            max_text_len,
            font_unit,
            pixel_format,
            background,
        )
    }


    pub fn write_char(
        &mut self,
        c: char,
        colors: &TextColors,
        char_writer: &mut impl CharWritable,
    ) -> KernelResult<bool> {
        if self.need_new_line()
        {
            return Ok(true);
        }

        let pos = Vector2D::new(self.texts.len() * self.font_unit.width(), 0);
        char_writer.write(
            self.text_buffs.as_mut_slice(),
            c,
            pos,
            colors,
            &mut self.buff_pixel_writer,
        )?;

        self.texts.push(c);

        Ok(false)
    }


    pub fn delete_last(
        &mut self,
    ) {
        self.texts.pop();
        let pos = Vector2D::new(self.texts.len() * self.font_unit.width(), 0);
        let draw_area = Rectangle::from_pos_and_size(pos, self.font_unit);

        self.buff_pixel_writer
            .fill_rect(self.text_buffs.as_mut_slice(), draw_area, &self.background)
    }


    #[inline(always)]
    pub fn need_new_line(&self) -> bool {
        self.max_text_len <= self.texts.len() || self
            .texts
            .last()
            .is_some_and(|c| *c == '\n')
    }


    pub fn frame_buff_lines(&self) -> Option<Vec<&[u8]>> {
        let mut lines = Vec::with_capacity(self.font_unit.height());
        for y in 0..self.font_unit.height() {
            lines.push(self.frame_buff_line(y));
        }

        Some(lines)
    }


    pub fn frame_buff_line(&self, y: usize) -> &[u8] {
        let origin = y * self.max_buff_width();
        &self.text_buffs[origin..origin + self.max_buff_width()]
    }


    #[inline]
    pub fn texts(&self) -> &[char] {
        self.texts.as_ref()
    }


    #[cfg(test)]
    fn buff_width(&self) -> usize {
        self.texts.len() * font_buff_width(&self.font_unit)
    }


    fn max_buff_width(&self) -> usize {
        self.max_text_len * font_buff_width(&self.font_unit)
    }


    fn new_with_buff(
        text_buffs: Vec<u8>,
        max_text_len: usize,
        font_unit: Size,
        pixel_format: PixelFormat,
        background: PixelColor,
    ) -> Self {
        let buff_size = text_buffer_size(max_text_len, &font_unit);

        Self {
            font_unit,
            text_buffs,
            buff_pixel_writer: BuffPixelWriter::new(buff_size, pixel_format),
            max_text_len,
            texts: Vec::with_capacity(max_text_len),
            background,
        }
    }
}


fn font_buff_width(font_size: &Size) -> usize {
    4 * font_size.width()
}


fn text_buffer_size(max_text_len: usize, font_unit: &Size) -> Size {
    Size::new(4 * max_text_len * font_unit.width(), font_unit.height())
}


fn new_text_row_buff(
    background: &PixelColor,
    font_unit: &Size,
    max_text_len: usize,
    pixel_format: PixelFormat,
) -> Vec<u8> {
    let mut pixel_mapper = EnumPixelMapper::new(pixel_format);
    let buff = pixel_mapper.convert_to_buff(background);

    vec![buff; max_text_len * font_unit.width() * font_unit.height()]
        .flatten()
        .to_vec()
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use common_lib::array::array_eq;
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::size::Size;

    use crate::gop::char::ascii_char_writer::AscIICharWriter;
    use crate::gop::char::char_writable::CharWritable;
    use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
    use crate::gop::pixel::mapper::PixelMapper;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::layers::text::colors::TextColors;
    use crate::layers::text::row::{new_text_row_buff, TextRow};

    fn padding_buff(
        padding: usize,
        background: &PixelColor,
        font_unit: &Size,
        pixel_format: PixelFormat,
        text_buff: &[u8],
    ) -> Vec<u8> {
        let v = *EnumPixelMapper::new(pixel_format).convert_to_buff(background);
        let mut buf = vec![v; padding * font_unit.width() * 4]
            .flatten()
            .to_vec();

        buf.extend_from_slice(text_buff);
        buf.resize(text_buff.len(), 0x00);
        buf
    }

    #[test]
    fn it_write_char() {
        let mut writer = AscIICharWriter::new();
        let mut row = TextRow::new(PixelColor::black(), writer.font_unit(), 5, PixelFormat::Rgb);


        row.write_char(
            'h',
            &TextColors::default().change_foreground(PixelColor::white()),
            &mut writer,
        )
            .unwrap();

        assert_eq!(row.buff_width(), writer.font_unit().width() * 4);
        assert!(row
            .frame_buff_lines()
            .is_some_and(|lines| lines.len() == writer.font_unit().height()));
    }


    #[test]
    fn it_reflect_color_with_rbg() {
        let background = PixelColor::yellow();
        let row = new_text_row_buff(&background, &Size::new(8, 16), 3, PixelFormat::Rgb);

        assert_eq!(row.len(), 96 * 16);
        assert!(row
            .chunks(4)
            .all(|pixel_buff| array_eq(pixel_buff, &[0xFF, 0xFF, 0x00, 0x00])));
    }


    #[test]
    fn it_reflect_color_with_bgr() {
        let background = PixelColor::yellow();
        let row = new_text_row_buff(&background, &Size::new(8, 16), 3, PixelFormat::Bgr);

        assert_eq!(row.len(), 96 * 16);
        assert!(row
            .chunks(4)
            .all(|pixel_buff| array_eq(pixel_buff, &[0x00, 0xFF, 0xFF, 0x00])));
    }


    #[test]
    fn it_padding_text_buff_not_resize() {
        let font_unit = Size::new(8, 16);
        let text_buff = vec![0; font_unit.width() * 8 * 4];
        let buf = padding_buff(
            8,
            &PixelColor::yellow(),
            &font_unit,
            PixelFormat::Rgb,
            text_buff.as_ref(),
        );

        assert_eq!(buf.len(), text_buff.len());
    }


    #[test]
    fn it_padding_text_buff() {
        const FONT_UNIT: Size = Size::new(8, 16);
        let buf_len = FONT_UNIT.width() * 8 * 4;

        let text_buff = vec![0; buf_len];
        let buf = padding_buff(
            8,
            &PixelColor::yellow(),
            &FONT_UNIT,
            PixelFormat::Rgb,
            text_buff.as_ref(),
        );


        const PADDING_LEN: usize = 8;
        assert!(array_eq(
            vec![[0xFF, 0xFF, 0x00, 0x00]; PADDING_LEN / 4]
                .flatten(),
            &buf[0..PADDING_LEN],
        ));
    }
}
