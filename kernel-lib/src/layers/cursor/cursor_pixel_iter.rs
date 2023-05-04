use alloc::vec::Vec;

use common_lib::math::vector::Vector2D;

use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_iter::PixelIter;
use crate::gop::pixel::Pixel;

#[derive(Debug, Clone)]
pub struct CursorPixelIter<'buff> {
    buff: &'buff Vec<Vec<u8>>,
    origin_pos: Vector2D<usize>,
    end_pos: Option<Vector2D<usize>>,
    cursor_color: PixelColor,
    border_color: PixelColor,
    x: usize,
    y: usize,
}


impl<'buff> PixelIter for CursorPixelIter<'buff> {}


impl<'buff> CursorPixelIter<'buff> {
    pub const fn new(
        buff: &'buff Vec<Vec<u8>>,
        origin_pos: Vector2D<usize>,
        end_pos: Option<Vector2D<usize>>,
        cursor_color: PixelColor,
        border_color: PixelColor,
    ) -> CursorPixelIter<'buff> {
        Self {
            buff,
            origin_pos,
            end_pos,
            cursor_color,
            border_color,
            x: 0,
            y: 0,
        }
    }
}


impl<'buff> Iterator for CursorPixelIter<'buff> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buff.len() <= self.y
            || self
                .end_pos
                .is_some_and(|pos| pos.y() < self.y)
        {
            return None;
        }

        if self.buff[0].len() <= self.x
            || self
                .end_pos
                .is_some_and(|pos| pos.x() < self.x)
        {
            self.y += 1;
            self.x = 0;
            self.next()
        } else {
            let pixel_color = cursor_color_at(
                char::from(self.buff[self.y][self.x]),
                self.cursor_color,
                self.border_color,
            );

            let pixel = Pixel::new(pixel_color, Vector2D::new(self.x, self.y) + self.origin_pos);

            self.x += 1;

            Some(pixel)
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
