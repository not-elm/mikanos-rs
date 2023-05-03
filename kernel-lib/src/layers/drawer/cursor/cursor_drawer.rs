use core::any::Any;

use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::layers::drawer::cursor::cursor_buffer::CursorBuffer;
use crate::layers::drawer::cursor::cursor_colors::CursorColors;
use crate::layers::drawer::LayerDrawable;

#[derive(Debug, Clone)]
pub struct CursorDrawer {
    cursor_buff: CursorBuffer,
    colors: CursorColors,
}


impl CursorDrawer {
    pub fn new(scale: Vector2D<usize>, colors: CursorColors) -> Self {
        Self {
            cursor_buff: CursorBuffer::new(scale),
            colors,
        }
    }

    pub fn cursor_size(&self) -> Size {
        self.cursor_buff.size()
    }


    pub fn set_color(&mut self, colors: CursorColors) {
        self.colors = colors
    }
}


impl LayerDrawable for CursorDrawer {
    fn draw_in_area(
        &mut self,
        pixels: &mut [u8],
        pixel_writer: &mut FrameBufferPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for pixel in self
            .cursor_buff
            .cursor_pixels(draw_area.origin(), Some(draw_area.end()), self.colors)
            .filter(|pixel| draw_area.with_in_pos(&pixel.pos()))
        {
            if let Some(color) = pixel.color() {
                unsafe {
                    pixel_writer.write(pixels, &pixel.pos(), &color)?;
                }
            }
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for CursorDrawer {
    fn default() -> Self {
        Self::new(Vector2D::unit(), CursorColors::default())
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::builder::Transform2DBuilder;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::layers::drawer::cursor::cursor_colors::CursorColors;
    use crate::layers::drawer::cursor::cursor_drawer::CursorDrawer;

    #[test]
    fn it_write_cursor_not_scale() {
        let cursor_color = PixelColor::blue();
        let border_color = PixelColor::yellow();
        let colors = CursorColors::new(cursor_color, border_color, Some(PixelColor::black()));

        let mut drawer = CursorDrawer::new(Vector2D::unit(), colors);
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
            .cursor_pixels_checked(&transform, cursor_color, border_color)
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

                assert_eq!(actual, expect);
            });
    }
}
