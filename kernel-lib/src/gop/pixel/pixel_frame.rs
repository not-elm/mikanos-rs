use alloc::boxed::Box;
use alloc::vec;

use common_lib::math::rectangle::Rectangle;

use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::gop::pixel::row::PixelRow;
use crate::gop::pixel::Pixel;
use crate::layers::drawer::rect_colors::RectColors;

pub struct PixelFrame<'buff> {
    pixels: Box<dyn Iterator<Item = Pixel> + 'buff>,
    transparent: Option<PixelColor>,
    converter: EnumPixelConverter,
    row_first: Option<Pixel>,
}


impl<'buff> PixelFrame<'buff> {
    pub fn new(
        mut pixels: impl Iterator<Item = Pixel> + 'buff,
        converter: EnumPixelConverter,
        transparent: Option<PixelColor>,
    ) -> PixelFrame<'buff> {
        let row_first = pixels.next();

        Self {
            pixels: Box::new(pixels),
            converter,
            transparent,
            row_first,
        }
    }


    pub fn rect(rect: Rectangle<usize>, colors: RectColors, converter: EnumPixelConverter) -> Self {
        let rect_iter = rect.points();

        Self::new(
            rect_iter.map(move |p| Pixel::new(Some(colors.foreground()), p)),
            converter,
            Some(DISPLAY_BACKGROUND_COLOR),
        )
    }
}


impl<'buff> Iterator for PixelFrame<'buff> {
    type Item = PixelRow<EnumPixelConverter>;

    fn next(&mut self) -> Option<Self::Item> {
        let row_first = self.row_first?;
        let mut row = vec![row_first];

        loop {
            if let Some(pixel) = self.pixels.next() {
                if pixel.pos.y() == row_first.pos.y() {
                    row.push(pixel);
                } else {
                    self.row_first = Some(pixel);
                    break;
                }
            } else {
                self.row_first = None;
                break;
            }
        }


        let row = PixelRow::new(row, self.converter.clone(), self.transparent);

        Some(row)
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::pixel_frame::PixelFrame;
    use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
    use crate::gop::pixel::row::PixelRow;
    use crate::layers::drawer::cursor::cursor_buffer::{CursorBuffer, CURSOR_HEIGHT, CURSOR_WIDTH};

    #[test]
    fn it_correct_width() {
        let buff = CursorBuffer::default();
        let pixels = buff.cursor_pixels(
            Vector2D::zeros(),
            None,
            PixelColor::white(),
            PixelColor::yellow(),
        );

        let mut pixel_frame = PixelFrame::new(
            pixels,
            EnumPixelConverter::new(PixelFormat::Bgr),
            Some(PixelColor::black()),
        );
        let row = pixel_frame.next();


        assert!(row.is_some_and(|row| row.pixels_len() == CURSOR_WIDTH));
    }


    #[test]
    fn it_correct_height() {
        let buff = CursorBuffer::default();
        let pixels = buff.cursor_pixels(
            Vector2D::zeros(),
            None,
            PixelColor::white(),
            PixelColor::yellow(),
        );

        let pixel_frame = PixelFrame::new(
            pixels,
            EnumPixelConverter::new(PixelFormat::Bgr),
            Some(PixelColor::black()),
        );
        let rows: Vec<PixelRow<EnumPixelConverter>> = pixel_frame.collect();

        assert_eq!(rows.len(), CURSOR_HEIGHT);
    }
}
