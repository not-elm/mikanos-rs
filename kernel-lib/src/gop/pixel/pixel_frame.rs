use alloc::vec;
use alloc::vec::Vec;

use common_lib::math::rectangle::Rectangle;

use crate::gop::pixel::Pixel;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::pixel_converter::PixelConvertable;
use crate::gop::pixel::row::PixelRow;

type Pixels = impl Iterator<Item=Pixel>;

pub struct PixelFrame {
    pixels: Pixels,
    index: usize,
}


impl PixelFrame {
    pub fn new(pixels: Pixels, converter: impl PixelConvertable) -> PixelFrame {
        // let pixels: Vec<Vec<Pixel>> = pixel_iter
        //     .into_iter()
        //     .group_by(|pixel: Pixel| pixel.pos().y())
        //     .map(|pixel| pixel.into_values())
        //     .collect();

        Self { pixels, index: 0 }
    }


    pub fn rect(rect: Rectangle<usize>, color: PixelColor) -> Self {
        let rect_iter = rect.points_unbound();

        Self::new(rect_iter.map(move |p| Pixel::new(Some(color), p)))
    }
}


impl Iterator for PixelFrame {
    type Item = Vec<PixelRow<>>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pixels.next()?;
        let mut v = vec![start];
        loop {
            if let Some(pixel) = self.pixels.next() {
                v.push(pixel);
            } else {
                break;
            }
        }

        Some(v)
    }
}


#[cfg(test)]
mod tests {
    use crate::layers::window::drawers::cursor::cursor_buffer::{
        CURSOR_HEIGHT, CURSOR_WIDTH, CursorBuffer,
    };

// #[test]
    // fn it_correct_width() {
    //     let buff = CursorBuffer::default();
    //     let pixels =
    //         buff.cursor_pixels(Vector2D::zeros(), PixelColor::white(), PixelColor::yellow());
    //     let pixel_frame = PixelFrame::new(pixels);
    //
    //     assert_eq!(pixel_frame.width(), CURSOR_WIDTH);
    // }
    //
    //
    // #[test]
    // fn it_correct_height() {
    //     let buff = CursorBuffer::default();
    //     let pixels =
    //         buff.cursor_pixels(Vector2D::zeros(), PixelColor::white(), PixelColor::yellow());
    //     let pixel_frame = PixelFrame::new(pixels);
    //
    //     assert_eq!(pixel_frame.height(), CURSOR_HEIGHT);
    // }
    //
    //
    // #[test]
    // fn it_from_rect() {
    //     let frame = PixelFrame::rect(
    //         Rectangle::from_size(Size::new(10, 10)),
    //         PixelColor::yellow(),
    //     );
    //
    //     assert_eq!(frame.width(), 10);
    //     assert_eq!(frame.height(), 10);
    // }
}
