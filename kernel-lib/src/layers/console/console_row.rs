use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::buff_pixel_writer::BuffPixelWriter;
use crate::layers::console::console_colors::ConsoleColors;

pub struct ConsoleRow {
    text_buffs: Vec<u8>,
    buff_pixel_writer: BuffPixelWriter,
    font_unit: Size,
    max_text_len: usize,
    current_text_len: usize,
    pixel_format: PixelFormat,
}


impl ConsoleRow {
    pub fn new(
        background: &PixelColor,
        font_unit: Size,
        max_text_len: usize,
        pixel_format: PixelFormat) -> Self {
        Self::new_with_buff(
            new_text_row_buff(background, &font_unit, max_text_len, pixel_format),
            max_text_len,
            font_unit,
            pixel_format,
            0,
        )
    }


    pub fn resize_text_len(&mut self, new_text_len: usize) {
        self.text_buffs
            .resize(text_buffer_length(new_text_len, &self.font_unit), 0);

        self.current_text_len = min(new_text_len, self.current_text_len);
        self.max_text_len = new_text_len;
        self.buff_pixel_writer = BuffPixelWriter::new(
            text_buffer_size(new_text_len, &self.font_unit),
            self.pixel_format,
        );
    }


    pub fn write_char(
        &mut self,
        c: char,
        colors: &ConsoleColors,
        char_writer: &mut impl CharWritable,
    ) -> KernelResult<bool> {
        if self.max_text_len <= self.current_text_len || c == '\n' {
            return Ok(true);
        }

        let pos = Vector2D::new(self.current_text_len * self.font_unit.width(), 0);
        char_writer.write(
            self.text_buffs.as_mut_slice(),
            c,
            pos,
            colors,
            &mut self.buff_pixel_writer,
        )?;

        self.current_text_len += 1;
        Ok(false)
    }


    pub fn frame_buff_lines(&self) -> Option<Vec<&[u8]>> {
        if self.current_text_len == 0 {
            return None;
        }

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

    #[cfg(test)]
    pub(crate) fn current_text_len(&self) -> usize {
        self.current_text_len
    }


    #[cfg(test)]
    pub(crate) fn max_text_len(&self) -> usize {
        self.max_text_len
    }

    #[cfg(test)]
    fn buff_width(&self) -> usize {
        self.current_text_len * font_buff_width(&self.font_unit)
    }


    fn max_buff_width(&self) -> usize {
        self.max_text_len * font_buff_width(&self.font_unit)
    }


    fn new_with_buff(
        text_buffs: Vec<u8>,
        max_text_len: usize,
        font_unit: Size,
        pixel_format: PixelFormat,
        current_text_len: usize,
    ) -> Self {
        let buff_size = text_buffer_size(max_text_len, &font_unit);

        Self {
            font_unit,
            text_buffs,
            buff_pixel_writer: BuffPixelWriter::new(buff_size, pixel_format),
            max_text_len,
            current_text_len,
            pixel_format,
        }
    }
}


fn font_buff_width(font_size: &Size) -> usize {
    4 * font_size.width()
}


fn text_buffer_size(max_text_len: usize, font_unit: &Size) -> Size {
    Size::new(4 * max_text_len * font_unit.width(), font_unit.height())
}


fn text_buffer_length(max_text_len: usize, font_unit: &Size) -> usize {
    4 * max_text_len * font_unit.width() * font_unit.height()
}


fn new_text_row_buff(
    background: &PixelColor,
    font_unit: &Size,
    max_text_len: usize,
    pixel_format: PixelFormat,
) -> Vec<u8> {
    vec![*EnumPixelMapper::new(pixel_format).convert_to_buff(background); max_text_len * font_unit.width() * font_unit.height()]
        .flatten()
        .to_vec()
}

#[cfg(test)]
mod tests {
    use common_lib::array::eq_array;
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::size::Size;

    use crate::gop::char::ascii_char_writer::AscIICharWriter;
    use crate::gop::char::char_writable::CharWritable;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::layers::console::console_colors::ConsoleColors;
    use crate::layers::console::console_row::{ConsoleRow, new_text_row_buff};

    #[test]
    fn it_write_char() {
        let mut writer = AscIICharWriter::new();
        let mut row = ConsoleRow::new(&PixelColor::black(), writer.font_unit(), 5, PixelFormat::Rgb);

        assert!(row
            .frame_buff_lines()
            .is_none());

        row.write_char(
            'h',
            &ConsoleColors::default().change_foreground(PixelColor::white()),
            &mut writer,
        )
            .unwrap();

        assert_eq!(row.buff_width(), writer.font_unit().width() * 4);
        assert!(row
            .frame_buff_lines()
            .is_some_and(|lines| lines.len() == writer.font_unit().height()));
    }


    #[test]
    fn it_over_size() {
        let mut writer = AscIICharWriter::new();
        let mut row = ConsoleRow::new(&PixelColor::black(), writer.font_unit(), 1, PixelFormat::Rgb);

        row.resize_text_len(2);
        assert_eq!(row.current_text_len, 0);
        assert_eq!(row.max_text_len, 2);
        assert!(!row
            .write_char(
                'h',
                &ConsoleColors::default().change_foreground(PixelColor::white()),
                &mut writer,
            )
            .unwrap());
        assert!(!row
            .write_char(
                'h',
                &ConsoleColors::default().change_foreground(PixelColor::white()),
                &mut writer,
            )
            .unwrap());
    }


    #[test]
    fn it_small_size() {
        let mut writer = AscIICharWriter::new();
        let mut row = ConsoleRow::new(&PixelColor::black(), writer.font_unit(), 5, PixelFormat::Rgb);
        row.write_char(
            'h',
            &ConsoleColors::default().change_foreground(PixelColor::white()),
            &mut writer,
        )
            .unwrap();
        row.write_char(
            'h',
            &ConsoleColors::default().change_foreground(PixelColor::white()),
            &mut writer,
        )
            .unwrap();
        row.write_char(
            'h',
            &ConsoleColors::default().change_foreground(PixelColor::white()),
            &mut writer,
        )
            .unwrap();

        row.resize_text_len(2);
        assert_eq!(row.current_text_len, 2);
        assert_eq!(row.max_text_len, 2);
        assert!(row
            .write_char(
                'h',
                &ConsoleColors::default().change_foreground(PixelColor::white()),
                &mut writer,
            )
            .unwrap())
    }


    #[test]
    fn it_reflect_color_with_rbg() {
        let background = PixelColor::yellow();
        let row = new_text_row_buff(&background, &Size::new(8, 16), 3, PixelFormat::Rgb);

        assert_eq!(row.len(), 96 * 16);
        assert!(row.chunks(4).all(|pixel_buff| eq_array(pixel_buff, &[0xFF, 0xFF, 0x00, 0x00])));
    }


    #[test]
    fn it_reflect_color_with_bgr() {
        let background = PixelColor::yellow();
        let row = new_text_row_buff(&background, &Size::new(8, 16), 3, PixelFormat::Bgr);

        assert_eq!(row.len(), 96 * 16);
        assert!(row.chunks(4).all(|pixel_buff| eq_array(pixel_buff, &[0x00, 0xFF, 0xFF, 0x00])));
    }
}
