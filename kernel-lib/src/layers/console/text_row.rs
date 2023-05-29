use crate::gop::char::ascii::ascii_char::AsciiChar;
use crate::gop::char::ascii::ascii_char_buff::{AsciiCharBuff, CharLines};

#[derive(Debug, PartialEq, Eq)]
pub enum TextRowStatus {
    Appended,
    RequestNewLine,
}


#[derive(Debug)]
pub struct TextRow {
    texts: CharLines,
    max_text_len: usize,
    current_text_len: usize,
    need_new_line: bool,
}


impl TextRow {
    pub fn new(max_text_len: usize) -> Self {
        Self {
            texts: Default::default(),
            max_text_len,
            current_text_len: 0,
            need_new_line: false,
        }
    }


    pub fn text_colors(&self) -> &CharLines {
        &self.texts
    }


    pub fn append_char(&mut self, c: char) -> TextRowStatus {
        if self.is_limit() || self.is_new_line() {
            return TextRowStatus::RequestNewLine;
        }

        if c == '\n' {
            self.need_new_line = true;
            return TextRowStatus::Appended;
        }

        self.push_char_lines(c);
        self.current_text_len += 1;

        TextRowStatus::Appended
    }


    fn push_char_lines(&mut self, c: char) {
        if let Some(ascii) = AsciiChar::new(c) {
            AsciiCharBuff::new(ascii)
                .enumerate()
                .for_each(|(y, line_colors)| {
                    self.texts[y].extend(line_colors);
                });
        }
    }


    fn is_limit(&self) -> bool {
        self.max_text_len <= self.current_text_len
    }


    fn is_new_line(&self) -> bool {
        self.need_new_line
    }
}


#[cfg(test)]
mod tests {
    use crate::layers::console::text_row::TextRowStatus;

    use super::TextRow;

    #[test]
    fn it_limit_texts_2() {
        let mut row = TextRow::new(2);
        assert_eq!(row.append_char('a'), TextRowStatus::Appended);
        assert_eq!(row.append_char('b'), TextRowStatus::Appended);
        assert_eq!(row.append_char('c'), TextRowStatus::RequestNewLine);
    }


    #[test]
    fn it_append_new_line() {
        let mut row = TextRow::new(3);
        assert_eq!(row.append_char('a'), TextRowStatus::Appended);
        assert_eq!(row.append_char('\n'), TextRowStatus::Appended);
        assert_eq!(row.append_char('c'), TextRowStatus::RequestNewLine);
    }
}
