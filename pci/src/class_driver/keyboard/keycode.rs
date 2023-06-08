#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Keycode(u8);


impl Keycode {
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }


    pub fn char(&self) -> char{
        const CHARS : [char; 29] = [
            '\0', '\0', '\0', '\0', 'a',  'b',  'c',  'd',  'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
            'm', 'n', 'o', 'p', 'q', 'r', 's', 'u', 'v', 'w', 'x', 'y', 'z'
        ];

        CHARS[self.0 as usize]
    }
}



