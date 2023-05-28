use alloc::vec::Vec;

use crate::gop::pixel::pixel_color::PixelColor;

use super::ascii_char::AsciiChar;

#[derive(Debug)]
pub struct AsciiCharBuff {
    y: usize,
    ascii: AsciiChar,
}

pub type CharLine = Vec<Option<PixelColor>>;
pub type CharLines = Vec<CharLine>;

impl AsciiCharBuff {
    pub fn new(ascii: AsciiChar) -> Self {
        Self { y: 0, ascii }
    }


    pub fn into_text_lines(self) -> CharLines {
        self.into_iter().collect()
    }
}


impl Iterator for AsciiCharBuff {
    type Item = Vec<Option<PixelColor>>;

    fn next(&mut self) -> Option<Self::Item> {
        if 16 <= self.y {
            return None;
        }


        let mut line_colors = Vec::<Option<PixelColor>>::new();

        let line_bits = self
            .ascii
            .font_line_bits(self.y)?;

        add_vec(&mut line_colors, line_bits);
        self.y += 1;

        Some(line_colors)
    }
}


fn add_vec(line_colors: &mut Vec<Option<PixelColor>>, line_bits: u8) {
    for x in 0..8 {
        let is_need_write_bit = ((line_bits << x) & 0x80u8) != 0;

        line_colors.push(if is_need_write_bit {
            Some(PixelColor::white())
        } else {
            None
        });
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::gop::{char::ascii::ascii_char::AsciiChar, pixel::pixel_color::PixelColor};

    use super::AsciiCharBuff;

    #[test]
    fn it_pixel_colors_a() {
        let buff = AsciiCharBuff::new(AsciiChar::new('A').unwrap());

        const A: [&[u8; 8]; 16] = [
            b"........",
            b"...@....",
            b"...@....",
            b"..@.@...",
            b"..@.@...",
            b"..@.@...",
            b".@...@..",
            b".@...@..",
            b".@...@..",
            b".@@@@@..",
            b"@.....@.",
            b"@.....@.",
            b"@.....@.",
            b"@.....@.",
            b"........",
            b"........",
        ];
        let vec: Vec<Vec<Option<PixelColor>>> = buff.collect();


        assert_eq!(vec[0].len(), 8);
        assert_eq!(vec.len(), 16);


        let colors: Vec<Option<PixelColor>> = vec
            .into_iter()
            .flatten()
            .collect();

        let expect_colors = A
            .into_iter()
            .flatten()
            .map(|c| {
                let c = char::from(*c);
                if c == '.' {
                    None
                } else {
                    Some(PixelColor::white())
                }
            });


        let all_sames = colors
            .into_iter()
            .zip(expect_colors)
            .all(|(color1, color2)| color1 == color2);

        assert!(all_sames);
    }
}
