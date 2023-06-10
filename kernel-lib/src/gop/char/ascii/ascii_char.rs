use crate::gop::font::{convert_to_ascii, get_font_from};

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AsciiChar(char);


impl AsciiChar {
    pub fn new(c: char) -> Option<Self> {
        Some(Self(convert_to_ascii(c)?))
    }


    pub const fn char(&self) -> char {
        self.0
    }


    pub fn font_raw_ptr(&self) -> Option<*mut u8> {
        get_font_from(self.char())
    }


    pub fn font_line_bits(&self, y: usize) -> Option<u8> {
        let raw = self.font_raw_ptr()?;

        if raw.is_null() {
            return None;
        }


        let raw = unsafe { raw.add(y) };
        if raw.is_null() {
            return None;
        }

        Some(unsafe { *raw })
    }
}


#[cfg(test)]
mod tests {
    use crate::gop::char::ascii::ascii_char::AsciiChar;

    #[test]
    fn it_convert_all_ascii_chars() {
        for c in 'a'..='\n' {
            assert!(AsciiChar::new(c).is_some_and(|ascii| ascii.char() == c))
        }
    }


    #[test]
    fn it_get_font_raw_ptrs() {
        for c in 'a'..='\n' {
            assert!(AsciiChar::new(c)
                .unwrap()
                .font_raw_ptr()
                .is_some());
        }
    }


    #[test]
    fn it_get_lines() {
        let ascii = AsciiChar::new('A').unwrap();

        for y in 0..16 {
            assert!(ascii
                .font_line_bits(y)
                .is_some());
        }
    }
}
