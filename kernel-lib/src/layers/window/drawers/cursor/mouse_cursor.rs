use core::any::Any;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::drawers::cursor::cursor_buffer::CursorBuffer;
use crate::layers::window::WindowDrawable;

#[derive(Debug, Clone)]
pub struct MouseCursorDrawer {
    cursor_buff: CursorBuffer,
    color: PixelColor,
    border_color: PixelColor,
}


impl MouseCursorDrawer {
    pub fn new(scale: Vector2D<usize>, color: PixelColor, border_color: PixelColor) -> Self {
        Self {
            cursor_buff: CursorBuffer::new(scale),
            color,
            border_color,
        }
    }

    pub fn cursor_size(&self) -> Size {
        self.cursor_buff.size()
    }


    pub fn set_color(&mut self, color: PixelColor) {
        self.color = color
    }


    pub fn set_border_color(&mut self, border_color: PixelColor) {
        self.border_color = border_color;
    }
}


impl WindowDrawable for MouseCursorDrawer {
    fn draw(&mut self, transform: &Transform2D, writer: &mut dyn PixelWritable) -> KernelResult {
        for pixel in self
            .cursor_buff
            .cursor_pixels(transform, self.color, self.border_color)?
        {
            if let Some(color) = pixel.color() {
                unsafe { writer.write(pixel.pos().x(), pixel.pos().y(), &color)? };
            }
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for MouseCursorDrawer {
    fn default() -> Self {
        Self::new(Vector2D::unit(), PixelColor::white(), PixelColor::black())
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::builder::Transform2DBuilder;

    use crate::gop::pixel::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::layers::window::drawers::cursor::mouse_cursor::MouseCursorDrawer;
    use crate::layers::window::drawers::WindowDrawable;

    #[test]
    fn it_write_cursor_not_scale() {
        let cursor_color = PixelColor::blue();
        let border_color = PixelColor::yellow();
        let mut drawer = MouseCursorDrawer::new(Vector2D::unit(), cursor_color, border_color);
        let mut writer = MockBufferPixelWriter::new(
            drawer.cursor_size().width() * 4,
            drawer.cursor_size().height() * 4,
        );

        let transform = Transform2DBuilder::new()
            .size(Size::new(
                drawer.cursor_size().width() * 4,
                drawer.cursor_size().height() * 4,
            ))
            .build();

        assert!(drawer
            .draw(&transform, &mut writer)
            .is_ok());

        let pixels = drawer
            .cursor_buff
            .cursor_pixels(&transform, cursor_color, border_color)
            .unwrap();

        let points: Vec<Vector2D<usize>> = drawer
            .cursor_size()
            .points()
            .collect();

        points
            .into_iter()
            .zip(pixels)
            .for_each(|(point, cursor_pixel)| {
                let actual = writer.pixel_at(point.x(), point.y());

                let expect = cursor_pixel
                    .color()
                    .unwrap_or(PixelColor::black());
                if actual == PixelColor::black() && expect == PixelColor::white() {
                    let x = point.y();
                }
                assert_eq!(actual, expect);
            });
    }
    //
    //
    // #[test]
    // fn it_write_cursor_scale2() {
    //     let mut drawer = MouseCursorDrawer::new(
    //         Vector2D::new(2, 2),
    //         PixelColor::white(),
    //         PixelColor::black(),
    //     );
    //     let mut writer = MockBufferPixelWriter::new(100, 100);
    //     assert!(drawer
    //         .draw(
    //             &Transform2DBuilder::new()
    //                 .size(Size::new(100, 100))
    //                 .build(),
    //             &mut writer,
    //         )
    //         .is_ok());
    //
    //     CURSOR_SHAPE
    //         .iter()
    //         .flat_map(|row| {
    //             row.iter()
    //                 .flat_map(|pixel| vec![pixel, pixel])
    //         })
    //         .flat_map(|row| vec![row, row])
    //         .enumerate()
    //         .for_each(|(i, p)| {
    //             let x = i % (CURSOR_WIDTH * 2);
    //             let y = i / (CURSOR_WIDTH * 2);
    //             assert_eq!(
    //                 writer.pixel_at(x, y),
    //                 cursor_color_at(x, y, PixelColor::white(),
    // PixelColor::black())
    // .unwrap_or(PixelColor::black())             );
    //         });
    //     for y in 0..CURSOR_HEIGHT {}
    // }
}
