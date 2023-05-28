use crate::gop::char::ascii::ascii_char_buff::CharLines;

use alloc::vec::Vec;
use common_lib::math::size::Size;

use super::text_row::{TextRow, TextRowStatus};

#[derive(Debug)]
pub struct TextFrame {
    rows: Vec<TextRow>,
    text_frame_size: Size,
}


impl TextFrame {
    pub fn new(text_frame_size: Size) -> Self {
        let mut me = Self {
            rows: Vec::with_capacity(text_frame_size.height()),
            text_frame_size,
        };

        me.add_row();
        me
    }


    pub fn rows_ref(&self) -> &Vec<TextRow> {
        &self.rows
    }


    pub fn update_string(&mut self, s: &str) {
        self.rows.remove(0);
        self.add_row();
        self.append_string(s);
    }


    pub fn append_string(&mut self, s: &str) {
        s.chars()
            .for_each(|c| self.append_char(c));
    }


    fn append_char(&mut self, c: char) {
        if TextRowStatus::RequestNewLine
            == self
                .rows
                .last_mut()
                .unwrap()
                .append_char(c)
        {
            self.new_line();
            self.append_char(c);
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


    fn add_row(&mut self) {
        self.rows
            .push(TextRow::new(self.text_frame_size.width()));
    }
}


#[cfg(test)]
mod tests {
    use common_lib::math::size::Size;

    use super::TextFrame;

    #[test]
    fn it_new_line_when_limit() {
        let mut frame = TextFrame::new(Size::new(3, 2));
        frame.append_char('A');
        assert_eq!(frame.rows.len(), 1);
        frame.append_char('A');
        assert_eq!(frame.rows.len(), 1);
        frame.append_char('A');
        assert_eq!(frame.rows.len(), 1);
        frame.append_char('A');
        assert_eq!(frame.rows.len(), 2);
    }


    #[test]
    fn it_new_line_when_append_new_line() {
        let mut frame = TextFrame::new(Size::new(3, 2));
        frame.append_char('A');
        assert_eq!(frame.rows.len(), 1);
        frame.append_char('\n');
        assert_eq!(frame.rows.len(), 1);
        frame.append_char('A');
        assert_eq!(frame.rows.len(), 2);
    }


    #[test]
    fn it_scroll() {
        let mut frame = TextFrame::new(Size::new(3, 2));
        frame.append_char('\n');
        frame.append_char('\n');
        frame.append_char('\n');
        frame.append_char('\n');

        assert_eq!(frame.rows.len(), 2);
    }
}
