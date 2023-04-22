use alloc::vec::Vec;

use crate::gop::font::convert_to_ascii;
use common_lib::math::vector::Vector2D;

use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone)]
pub struct CursorPixel {
    color: Option<PixelColor>,
    pos: Vector2D<usize>,
}


impl CursorPixel {
    pub const fn new(color: Option<PixelColor>, pos: Vector2D<usize>) -> Self {
        Self { color, pos }
    }

    pub fn color(&self) -> Option<PixelColor> {
        self.color
    }


    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
    }
}


#[derive(Debug, Clone)]
pub struct CursorPixelIter<'buff> {
    buff: &'buff Vec<Vec<u8>>,
    origin_pos: Vector2D<usize>,
    cursor_color: PixelColor,
    border_color: PixelColor,
    x: usize,
    y: usize,
}


impl<'buff> CursorPixelIter<'buff> {
    pub const fn new(
        buff: &'buff Vec<Vec<u8>>,
        origin_pos: Vector2D<usize>,
        cursor_color: PixelColor,
        border_color: PixelColor,
    ) -> CursorPixelIter<'buff> {
        Self {
            buff,
            origin_pos,
            cursor_color,
            border_color,
            x: 0,
            y: 0,
        }
    }
}


impl<'buff> Iterator for CursorPixelIter<'buff> {
    type Item = CursorPixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buff.len() <= self.y {
            return None;
        }

        if self.buff[0].len() <= self.x {
            self.y += 1;
            self.x = 0;
            self.next()
        } else {
            let pixel_color = cursor_color_at(
                char::from(self.buff[self.y][self.x]),
                self.cursor_color,
                self.border_color,
            );

            self.x += 1;

            Some(CursorPixel::new(
                pixel_color,
                Vector2D::new(self.x - 1, self.y) + self.origin_pos,
            ))
        }
    }
}


pub(crate) fn cursor_color_at(
    c: char,
    cursor_color: PixelColor,
    border_color: PixelColor,
) -> Option<PixelColor> {
    if c == '@' {
        Some(border_color)
    } else if c == '.' {
        Some(cursor_color)
    } else {
        None
    }
}
