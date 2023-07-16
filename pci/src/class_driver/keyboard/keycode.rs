#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Keycode {
    ArrowUp,
    ArrowDown,
    Ascii(char),
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct KeycodeParser(u8);


impl KeycodeParser {
    pub const fn new(raw: u8) -> Self {
        Self(raw)
    }


    pub fn upper_char(&self) -> Option<Keycode> {
        self.char().map(|c| match c {
            Keycode::Ascii(code) => Keycode::Ascii(code.to_ascii_uppercase()),
            keycode => keycode,
        })
    }


    pub fn char(&self) -> Option<Keycode> {
        match self.0 {
            0x51 => Some(Keycode::ArrowDown),
            0x52 => Some(Keycode::ArrowUp),
            _ => {
                const NUL: char = '\0';
                const ENT: char = '\r';
                const ESC: char = '\x1B';
                const DEL: char = '\x7F';
                const TAB: char = '\x09';
                const SPC: char = '\x20';
                const HYP: char = '\x2D';
                const EQL: char = '\x3D';
                const LBR: char = '\x7B'; // {
                const RBR: char = '\x7D'; // }

                const CHARS: [char; 49] = [
                    NUL, NUL, NUL, NUL, 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
                    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2',
                    '3', '4', '5', '6', '7', '8', '9', '0', ENT, ESC, DEL, TAB, SPC, HYP, EQL, LBR,
                    RBR,
                ];

                CHARS
                    .get(self.0 as usize)
                    .copied()
                    .and_then(|c| if c != NUL { Some(c) } else { None })
                    .map(Keycode::Ascii)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::class_driver::keyboard::keycode::Keycode::Ascii;
    use crate::class_driver::keyboard::keycode::{Keycode, KeycodeParser};

    #[test]
    fn it_get_alphabets() {
        ('a'..='z')
            .enumerate()
            .for_each(|(i, alphabet)| {
                assert_eq!(
                    KeycodeParser::new((0x04 + i) as u8).char(),
                    Some(Keycode::Ascii(alphabet))
                );
            });
    }


    #[test]
    fn it_get_nums() {
        (1..=9).for_each(|num| {
            assert_eq!(
                KeycodeParser::new((0x1E + num - 1) as u8).char(),
                Some(Ascii(char::from_digit(num, 10).unwrap()))
            );
        });


        assert_eq!(KeycodeParser::new(0x27).char(), Some(Ascii('0')));
    }


    #[test]
    fn it_none_if_null_character() {
        assert_eq!(KeycodeParser::new(0x00).char(), None);
    }
}
