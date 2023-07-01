use alloc::vec::Vec;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::text::colors::TextColors;
use crate::layers::text::command::CommandAction;
use crate::layers::text::config::TextConfig;
use crate::layers::text::row::TextRow;

pub struct TextFrame {
    rows: Vec<TextRow>,
    text_frame_size: Size,
    pixel_format: PixelFormat,
    char_writer: AscIICharWriter,
    config: TextConfig,
}


impl TextFrame {
    pub fn new(
        char_writer: AscIICharWriter,
        text_frame_size: Size,
        pixel_format: PixelFormat,
        config: TextConfig,
    ) -> KernelResult<TextFrame> {
        let mut me = Self {
            rows: Vec::new(),
            char_writer,
            text_frame_size,
            pixel_format,
            config,
        };
        me.add_row(true)?;
        Ok(me)
    }


    pub fn text_cursor_pos(&self) -> Vector2D<usize> {
        let text_len = self
            .rows
            .last()
            .unwrap()
            .texts()
            .len();
        let x = text_len * self.config.text_unit.width();
        let y = (self.rows.len() - 1) * self.config.text_unit.height();

        Vector2D::new(x, y)
    }


    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.rows.remove(0);
        self.add_row(true)?;
        self.append_string(str)
    }


    pub fn append_string(&mut self, str: &str) -> KernelResult {
        if str.is_empty() {
            return Ok(());
        }

        for c in str.chars() {
            if self.write_char(self.config.colors, c)? {
                self.execute_command_if_need()?;
                self.new_line(true)?;
                self.write_char(self.config.colors, c)?;
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
        if self.need_back_line() {
            self.rows
                .remove(self.rows.len() - 1);
        } else {
            self.rows
                .last_mut()
                .unwrap()
                .delete_last();
        }
    }


    pub fn change_colors(&mut self, colors: TextColors) -> KernelResult {
        self.config.colors = colors;
        let mut rows = Vec::with_capacity(self.rows.len());

        for i in 0..self.rows.len() {
            let mut row = self.new_row_with_prefix_if_exists()?;
            for c in self.rows[i].texts() {
                row.write_char(*c, &self.config.colors, &mut self.char_writer)?;
            }
            rows.push(row);
        }

        self.rows = rows;

        Ok(())
    }


    fn need_back_line(&self) -> bool {
        let row = self.rows.last().unwrap();
        let text_len = row.texts().len();
        if self.config.exists_prefix() {
            text_len <= 1
        } else {
            text_len == 0
        }
    }


    fn new_line(&mut self, with_prefix: bool) -> KernelResult {
        if self.text_frame_size.height() <= self.rows.len() {
            self.scroll(with_prefix)?;
        } else {
            self.add_row(with_prefix)?;
        }

        Ok(())
    }


    fn execute_command_if_need(&mut self) -> KernelResult {
        if self
            .config
            .not_exists_command()
        {
            return Ok(());
        }


        let chars: Vec<char> = self
            .rows
            .last()
            .unwrap()
            .texts()
            .to_vec();

        let i = self
            .config
            .prefix
            .map(|_| 1)
            .unwrap_or(0);

        match self
            .config
            .try_execute_command(&chars[i..])
        {
            Ok(action) => {
                self.action(action)?;
            }
            Err(message) => {
                self.new_line(false)?;
                let colors = TextColors::new(
                    PixelColor::red(),
                    self.config
                        .colors
                        .background(),
                );
                self.output(&message, colors)?;
            }
        }

        Ok(())
    }


    fn action(&mut self, action: CommandAction) -> KernelResult {
        match action {
            CommandAction::Clear => {
                self.rows.clear();
            }
            CommandAction::Output(output) => {
                self.new_line(false)?;
                self.output(&output, self.config.colors)?;
            }
        }

        Ok(())
    }


    fn output(&mut self, output: &str, colors: TextColors) -> KernelResult {
        for c in output.chars() {
            if self.write_char(colors, c)? {
                self.new_line(false)?;
                self.write_char(colors, c)?;
            }
        }

        Ok(())
    }


    fn scroll(&mut self, with_prefix: bool) -> KernelResult {
        if self.config.scrollable {
            self.rows.remove(0);
            self.add_row(with_prefix)?;
        }

        Ok(())
    }


    fn write_char(&mut self, colors: TextColors, c: char) -> KernelResult<bool> {
        self.rows
            .last_mut()
            .unwrap()
            .write_char(c, &colors, &mut self.char_writer)
    }


    #[inline]
    fn add_row(&mut self, with_prefix: bool) -> KernelResult {
        let row = if with_prefix {
            self.new_row_with_prefix_if_exists()?
        } else {
            self.new_row()
        };

        self.rows.push(row);

        Ok(())
    }


    #[inline]
    fn new_row_with_prefix_if_exists(&mut self) -> KernelResult<TextRow> {
        let mut row = self.new_row();
        if let Some(prefix) = self.config.prefix {
            row.write_char(prefix, &self.config.colors, &mut self.char_writer)?;
        }

        Ok(row)
    }


    fn new_row(&self) -> TextRow {
        let background = self
            .config
            .colors
            .background();

        TextRow::new(
            background,
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

    use crate::gop;
    use crate::gop::char::ascii_char_writer::AscIICharWriter;
    use crate::layers::text::config;
    use crate::layers::text::frame::TextFrame;

    #[test]
    fn it_keeping_max_lines() {
        gop::test_init();

        let config = config::Builder::new()
            .set_scrollable()
            .build();

        let mut frame = TextFrame::new(
            AscIICharWriter::new(),
            Size::new(100, 3),
            PixelFormat::Rgb,
            config,
        )
        .unwrap();
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
        gop::test_init();

        let config = config::Builder::new()
            .set_scrollable()
            .build();

        let mut frame = TextFrame::new(
            AscIICharWriter::new(),
            Size::new(3, 3),
            PixelFormat::Rgb,
            config,
        )
        .unwrap();

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
        gop::test_init();

        let config = config::Builder::new()
            .set_scrollable()
            .build();

        let mut frame = TextFrame::new(
            AscIICharWriter::new(),
            Size::new(3, 3),
            PixelFormat::Rgb,
            config,
        )
        .unwrap();

        frame
            .append_string("Hello")
            .unwrap();

        assert_eq!(frame.frame_buff_lines().len(), 2);
    }
}
