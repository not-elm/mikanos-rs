use alloc::vec::Vec;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::layers::text::colors::TextColors;
use crate::layers::text::row::TextRow;
use crate::serial_println;

pub struct TextFrame<Char> {
    rows: Vec<TextRow>,
    colors: TextColors,
    text_frame_size: Size,
    text_unit: Size,
    pixel_format: PixelFormat,
    char_writer: Char,
    scrollable: bool,
    prefix: Option<char>,
}


impl<Char: CharWritable> TextFrame<Char> {
    pub fn new(
        colors: TextColors,
        char_writer: Char,
        text_frame_size: Size,
        text_unit: Size,
        pixel_format: PixelFormat,
        scrollable: bool,
        prefix: Option<char>,
    ) -> KernelResult<TextFrame<Char>> {
        let mut me = Self {
            rows: Vec::new(),
            char_writer,
            colors,
            text_frame_size,
            text_unit,
            pixel_format,
            scrollable,
            prefix,
        };
        me.add_row()?;
        Ok(me)
    }


    pub fn text_cursor_pos(&self) -> Vector2D<usize> {
        let x = self.rows.last().unwrap().texts().len() * self.text_unit.width();
        let y = (self.rows.len() - 1) * self.text_unit.height();

        Vector2D::new(x, y)
    }


    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.rows.remove(0);
        self.add_row()?;
        self.append_string(str)
    }


    pub fn append_string(&mut self, str: &str) -> KernelResult {
        if str.is_empty() {
            return Ok(());
        }

        for c in str.chars() {
            if self.write_char(c)? {
                self.new_line()?;
                self.write_char(c)?;
            }
        }

        Ok(())
    }


    #[inline]
    pub fn frame_buff_lines(&self) -> Vec<Vec<&[u8]>> {
        self.rows
            .iter()
            .filter_map(|row| row.frame_buff_lines())
            .collect()
    }


    pub fn delete_last(&mut self) {
        self.rows
            .last_mut()
            .unwrap()
            .delete_last()
    }


    pub fn change_colors(&mut self, colors: TextColors) -> KernelResult {
        self.colors = colors;
        let mut rows = Vec::with_capacity(self.rows.len());

        for i in 0..self.rows.len() {
            let mut row = self.new_row()?;
            for c in self.rows[i].texts() {
                row.write_char(*c, &self.colors, &mut self.char_writer)?;
            }
            rows.push(row);
        }

        self.rows = rows;

        Ok(())
    }


    fn new_line(&mut self) -> KernelResult {
        if self.text_frame_size.height() <= self.rows.len() {
            self.scroll()?;
        } else {
            self.add_row()?;
        }

        Ok(())
    }


    fn scroll(&mut self) -> KernelResult {
        if self.scrollable {
            self.rows.remove(0);
            self.add_row()?;
        }

        Ok(())
    }


    fn write_char(&mut self, c: char) -> KernelResult<bool> {
        self.rows
            .last_mut()
            .unwrap()
            .write_char(c, &self.colors, &mut self.char_writer)
    }


    #[inline]
    fn add_row(&mut self) -> KernelResult {
        let row = self.new_row()?;
        self.rows.push(row);
        
        Ok(())
    }


    #[inline]
    fn new_row(&mut self) -> KernelResult<TextRow> {
        let mut row = TextRow::new(
            *self.colors.background_ref(),
            self.char_writer.font_unit(),
            self.text_frame_size.width(),
            self.pixel_format,
        );

        if let Some(prefix) = self.prefix {
            row.write_char(prefix, &self.colors, &mut self.char_writer)?;
        }

        Ok(row)
    }
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::size::Size;

    use crate::gop::char::ascii_char_writer::AscIICharWriter;
    use crate::layers::text::colors::TextColors;
    use crate::layers::text::frame::TextFrame;

    #[test]
    fn it_keeping_max_lines() {
        let mut frame = TextFrame::new(
            TextColors::default(),
            AscIICharWriter::new(),
            Size::new(100, 3),
            Size::new(8, 16),
            PixelFormat::Rgb,
            true,
            None,
        ).unwrap();
        frame.new_line().unwrap();
        frame.new_line().unwrap();
        frame.new_line().unwrap();
        assert_eq!(frame.rows.len(), 3);
        frame.new_line().unwrap();
        frame.new_line().unwrap();
        assert_eq!(frame.rows.len(), 3);
    }


    #[test]
    fn it_new_line() {
        let mut frame = TextFrame::new(
            TextColors::default(),
            AscIICharWriter::new(),
            Size::new(3, 3),
            Size::new(8, 16),
            PixelFormat::Rgb,
            true,
            None,
        ).unwrap();

        frame
            .append_string("ABC")
            .unwrap();
        assert_eq!(frame.rows.len(), 1);

        frame
            .append_string("A")
            .unwrap();
        assert_eq!(frame.rows.len(), 2);
    }


    #[test]
    fn it_frame_buffer_lines_2_rows() {
        let mut frame = TextFrame::new(
            TextColors::default(),
            AscIICharWriter::new(),
            Size::new(3, 3),
            Size::new(8, 16),
            PixelFormat::Rgb,
            true,
            None,
        ).unwrap();

        frame
            .append_string("Hello")
            .unwrap();

        assert_eq!(frame.frame_buff_lines().len(), 2);
    }
}
