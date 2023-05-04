use alloc::vec::Vec;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::size::Size;

use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::layers::console::console_colors::ConsoleColors;
use crate::layers::console::console_row::ConsoleRow;

pub struct ConsoleFrame<Char> {
    rows: Vec<ConsoleRow>,
    colors: ConsoleColors,
    text_frame_size: Size,
    pixel_format: PixelFormat,
    char_writer: Char,
}


impl<Char: CharWritable> ConsoleFrame<Char> {
    pub fn new(
        colors: ConsoleColors,
        char_writer: Char,
        text_frame_size: Size,
        pixel_format: PixelFormat,
    ) -> Self {
        let mut me = Self {
            rows: Vec::new(),
            char_writer,
            colors,
            text_frame_size,
            pixel_format,
        };
        me.add_row();
        me
    }


    pub fn write_string(&mut self, str: &str) -> KernelResult {
        for c in str.chars() {
            if self.write_char(c)? {
                self.new_line();
                self.write_char(c)?;
            }
        }

        Ok(())
    }


    pub fn frame_buff_lines(&self) -> Vec<Vec<&[u8]>> {
        self.rows
            .iter()
            .filter_map(|row| row.frame_buff_lines())
            .collect()
    }


    pub fn resize_text_frame(&mut self, text_frame_size: Size) {
        let prev = self.text_frame_size;
        self.text_frame_size = text_frame_size;
        if text_frame_size.height() < prev.height() {
            self.rows
                .resize_with(text_frame_size.height(), || {
                    ConsoleRow::new(
                        self.char_writer.font_unit(),
                        text_frame_size.width(),
                        self.pixel_format,
                    )
                });
        }
        if text_frame_size.width() != prev.width() {
            self.rows
                .iter_mut()
                .for_each(|row| {
                    row.resize_text_len(text_frame_size.width());
                });
        }
    }


    fn new_line(&mut self) {
        if self.text_frame_size.height() <= self.rows.len() {
            self.scroll();
        } else {
            self.add_row();
        }
    }


    fn scroll(&mut self) {
        self.rows.remove(0);
        self.add_row();
    }


    fn write_char(&mut self, c: char) -> KernelResult<bool> {
        self.rows
            .last_mut()
            .unwrap()
            .write_char(c, self.colors.foreground(), &mut self.char_writer)
    }


    fn add_row(&mut self) {
        self.rows.push(self.new_row())
    }


    fn new_row(&self) -> ConsoleRow {
        ConsoleRow::new(
            self.char_writer.font_unit(),
            self.text_frame_size.width(),
            self.pixel_format,
        )
    }
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::size::Size;

    use crate::gop::char::ascii_char_writer::AscIICharWriter;
    use crate::layers::console::console_colors::ConsoleColors;
    use crate::layers::console::console_frame::ConsoleFrame;

    #[test]
    fn it_keeping_max_lines() {
        let mut frame = ConsoleFrame::new(
            ConsoleColors::default(),
            AscIICharWriter::new(),
            Size::new(100, 3),
            PixelFormat::Rgb,
        );
        frame.new_line();
        frame.new_line();
        frame.new_line();
        assert_eq!(frame.rows.len(), 3);
        frame.new_line();
        frame.new_line();
        assert_eq!(frame.rows.len(), 3);
    }


    #[test]
    fn it_new_line() {
        let mut frame = ConsoleFrame::new(
            ConsoleColors::default(),
            AscIICharWriter::new(),
            Size::new(3, 3),
            PixelFormat::Rgb,
        );
        frame
            .write_string("ABC")
            .unwrap();
        assert_eq!(frame.rows.len(), 1);

        frame
            .write_string("A")
            .unwrap();
        assert_eq!(frame.rows.len(), 2);
    }


    #[test]
    fn it_frame_buffer_lines_2_rows() {
        let mut frame = ConsoleFrame::new(
            ConsoleColors::default(),
            AscIICharWriter::new(),
            Size::new(3, 3),
            PixelFormat::Rgb,
        );
        frame
            .write_string("Hello")
            .unwrap();

        assert_eq!(frame.frame_buff_lines().len(), 2);
    }


    #[test]
    fn it_resize_to_over() {
        let mut frame = ConsoleFrame::new(
            ConsoleColors::default(),
            AscIICharWriter::new(),
            Size::new(3, 3),
            PixelFormat::Rgb,
        );
        frame.resize_text_frame(Size::new(5, 5));

        assert_eq!(frame.rows.len(), 1);
        assert_eq!(frame.text_frame_size, Size::new(5, 5));

        frame
            .write_string("hello")
            .unwrap();
        assert_eq!(frame.rows.len(), 1);

        assert_eq!(frame.rows[0].current_text_len(), 5);
        assert_eq!(frame.rows[0].max_text_len(), 5);

        frame
            .write_string("!")
            .unwrap();
        assert_eq!(frame.rows.len(), 2);
        assert_eq!(frame.rows[1].current_text_len(), 1);
        assert_eq!(frame.rows[1].max_text_len(), 5);
    }


    #[test]
    fn it_resize_to_small() {
        let mut frame = ConsoleFrame::new(
            ConsoleColors::default(),
            AscIICharWriter::new(),
            Size::new(3, 3),
            PixelFormat::Rgb,
        );
        frame
            .write_string("1234")
            .unwrap();
        frame
            .write_string("123")
            .unwrap();
        frame
            .write_string("12")
            .unwrap();

        frame.resize_text_frame(Size::new(1, 1));

        assert_eq!(frame.rows.len(), 1);
        assert_eq!(frame.text_frame_size, Size::new(1, 1));

        assert_eq!(frame.rows[0].current_text_len(), 1);
        assert_eq!(frame.rows[0].max_text_len(), 1);

        frame
            .write_string("!")
            .unwrap();
        assert_eq!(frame.rows.len(), 1);
        assert_eq!(frame.rows[0].current_text_len(), 1);
        assert_eq!(frame.rows[0].max_text_len(), 1);
    }
}
