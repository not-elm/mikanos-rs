use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use common_lib::iter::Grouping;
use common_lib::math::rectangle::Rectangle;

use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::pixel_converter::PixelConvertable;
use crate::gop::pixel::row::PixelRow;
use crate::gop::pixel::Pixel;

pub struct PixelFrame {
    pixels: Vec<Vec<Pixel>>,
    index: usize,
}


impl PixelFrame {
    pub fn new(pixel_iter: impl Iterator<Item = Pixel>) -> PixelFrame {
        let pixels: Vec<Vec<Pixel>> = pixel_iter
            .into_iter()
            .group_by(|pixel: Pixel| pixel.pos().y())
            .map(|pixel| pixel.into_values())
            .collect();

        Self { pixels, index: 0 }
    }


    pub fn rect(rect: Rectangle<usize>, color: PixelColor) -> Self {
        let rect_iter = rect.points_unbound();

        Self::new(rect_iter.map(|p| Pixel::new(Some(color), p)))
    }


    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }


    pub fn height(&self) -> usize {
        self.pixels.len()
    }


    pub fn into_pixels<Convert>(
        self,
        converter: Convert,
        transparent: Option<PixelColor>,
    ) -> Vec<PixelRow<Convert>>
    where
        Convert: PixelConvertable + Clone,
    {
        self.pixels
            .into_iter()
            .map(|row| PixelRow::new(row, converter.clone(), transparent))
            .collect()
    }
}


impl Index<usize> for PixelFrame {
    type Output = Vec<Pixel>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}


impl IndexMut<usize> for PixelFrame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}


impl Iterator for PixelFrame {
    type Item = Vec<Pixel>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixels.len() <= self.index {
            return None;
        }

        let row = self[self.index].clone();
        self.index += 1;
        Some(row)
    }
}


#[cfg(test)]
mod tests {
    use common_lib::math::rectangle::Rectangle;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::pixel_frame::PixelFrame;
    use crate::layers::window::drawers::cursor::cursor_buffer::{
        CursorBuffer, CURSOR_HEIGHT, CURSOR_WIDTH,
    };

    #[test]
    fn it_correct_width() {
        let buff = CursorBuffer::default();
        let pixels =
            buff.cursor_pixels(Vector2D::zeros(), PixelColor::white(), PixelColor::yellow());
        let pixel_frame = PixelFrame::new(pixels);

        assert_eq!(pixel_frame.width(), CURSOR_WIDTH);
    }


    #[test]
    fn it_correct_height() {
        let buff = CursorBuffer::default();
        let pixels =
            buff.cursor_pixels(Vector2D::zeros(), PixelColor::white(), PixelColor::yellow());
        let pixel_frame = PixelFrame::new(pixels);

        assert_eq!(pixel_frame.height(), CURSOR_HEIGHT);
    }


    #[test]
    fn it_from_rect() {
        let frame = PixelFrame::rect(
            Rectangle::from_size(Size::new(10, 10)),
            PixelColor::yellow(),
        );

        assert_eq!(frame.width(), 10);
        assert_eq!(frame.height(), 10);
    }
}
