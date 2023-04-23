use alloc::vec::Vec;
use common_lib::iter::Grouping;
use common_lib::math::vector::Vector2D;
use crate::gop::pixel::Pixel;
use crate::gop::pixel::pixel_iter::PixelIter;

pub struct PixelFrame {
    pixels: Vector2D<Vector2D<Pixel>>,
}


impl PixelFrame
{
    pub const fn new<Itera: PixelIter>(pixel_iter: impl Iterator<Item = Pixel>) -> PixelFrame{
        let pixel_iter = pixel_iter.peekable();

        let pixels: Vec<Vec<Pixel>> = pixel_iter
            .into_iter()
            .group_by(|pixel|{
                pixel
            })
            .collect();

        Self { pixels }
    }





}


    pub fn width(pixel_iter: &impl PixelIter) -> Option<usize> {
        let first = {pixel_iter.};

        let end_x = self.pixel_iter
            .take_while(|p|p.pos.y() == first.y())
            .last()?
            .pos.x();


        Some(end_x + 1)
    }


#[cfg(test)]
mod tests {
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::pixel_frame::PixelFrame;
    use crate::layers::window::drawers::cursor::cursor_buffer::{CursorBuffer, CURSOR_WIDTH};
    use common_lib::math::vector::Vector2D;

    #[test]
    fn it_correct_width() {
        let buff = CursorBuffer::default();
        let pixels =
            buff.cursor_pixels(Vector2D::zeros(), PixelColor::white(), PixelColor::yellow());
        let pixel_frame = PixelFrame::new(pixels);

        assert_eq!(pixel_frame.width(), CURSOR_WIDTH);
    }
}
