use alloc::boxed::Box;
use alloc::vec;

use common_lib::math::rectangle::Rectangle;

use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_row::PixelRow;
use crate::gop::pixel::Pixel;
use crate::layers::shape::shape_colors::ShapeColors;

pub struct PixelFrame<'buff> {
    pixels: Box<dyn Iterator<Item = Pixel> + 'buff>,
    transparent: Option<PixelColor>,
    converter: EnumPixelMapper,
    row_first: Option<Pixel>,
}


impl<'buff> PixelFrame<'buff> {
    pub fn new(
        mut pixels: impl Iterator<Item = Pixel> + 'buff,
        converter: EnumPixelMapper,
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


    pub fn rect(rect: Rectangle<usize>, colors: ShapeColors, converter: EnumPixelMapper) -> Self {
        let rect_iter = rect.points();

        Self::new(
            rect_iter.map(move |p| Pixel::new(Some(colors.foreground()), p)),
            converter,
            Some(DISPLAY_BACKGROUND_COLOR),
        )
    }
}


impl<'buff> Iterator for PixelFrame<'buff> {
    type Item = PixelRow<EnumPixelMapper>;

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
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::pixel_frame::PixelFrame;
    use crate::layers::cursor::cursor_buffer::{CursorBuffer, CURSOR_WIDTH};
    use crate::layers::cursor::cursor_colors::CursorColors;

    #[test]
    fn it_correct_width() {
        let buff = CursorBuffer::default();
        let pixels = buff.cursor_pixels(
            Vector2D::zeros(),
            None,
            CursorColors::new(
                PixelColor::white(),
                PixelColor::yellow(),
                Some(PixelColor::black()),
            ),
        );

        let mut pixel_frame = PixelFrame::new(
            pixels,
            EnumPixelMapper::new(PixelFormat::Bgr),
            Some(PixelColor::black()),
        );
        let row = pixel_frame.next();


        assert!(row.is_some_and(|row| row.pixels_len() == CURSOR_WIDTH));
    }
    //
    //
    // #[test]
    // fn it_correct_height() {
    //     let buff = CursorBuffer::default();
    //     let pixels = buff.cursor_pixels(
    //         Vector2D::zeros(),
    //         None,
    //         PixelColor::white(),
    //         PixelColor::yellow(),
    //     );
    //
    //     let pixel_frame = PixelFrame::new(
    //         pixels,
    //         EnumPixelMapper::new(PixelFormat::Bgr),
    //         Some(PixelColor::black()),
    //     );
    //     let rows: Vec<PixelRow<EnumPixelMapper>> = pixel_frame.collect();
    //
    //     assert_eq!(rows.len(), CURSOR_HEIGHT);
    // }
}
