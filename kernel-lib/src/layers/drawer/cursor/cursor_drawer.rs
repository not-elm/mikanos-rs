use alloc::vec::Vec;
use core::any::Any;

use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::{calc_pixel_pos_from_vec2d, calc_shadow_buffer_pixel_pos_from_vec2d};
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
use crate::layers::drawer::cursor::cursor_buffer::CursorBuffer;
use crate::layers::drawer::cursor::cursor_colors::CursorColors;
use crate::layers::drawer::LayerDrawable;

#[derive(Debug, Clone)]
pub struct CursorDrawer {
    cursor_buff: CursorBuffer,
    colors: CursorColors,
    converter: EnumPixelConverter,
}


impl CursorDrawer {
    pub fn new(scale: Vector2D<usize>, colors: CursorColors, pixel_format: PixelFormat) -> Self {
        Self {
            cursor_buff: CursorBuffer::new(scale),
            colors,
            converter: EnumPixelConverter::new(pixel_format),
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
        config: &FrameBufferConfig,
        layer_transform: &Transform2D,
        pixels: &mut [PixelColor],
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for p in self
            .cursor_buff
            .cursor_pixels(draw_area.origin(), Some(draw_area.end()), self.colors)
        {
            let i = calc_shadow_buffer_pixel_pos_from_vec2d(config, p.pos() + layer_transform.pos())?;
            p.color()
                .inspect(|color| pixels[i] = *color);
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for CursorDrawer {
    fn default() -> Self {
        Self::new(Vector2D::unit(), CursorColors::default(), PixelFormat::Rgb)
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::builder::Transform2DBuilder;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::writer::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::layers::drawer::cursor::cursor_colors::CursorColors;
    use crate::layers::drawer::cursor::cursor_drawer::CursorDrawer;
    use crate::layers::drawer::LayerDrawable;

    #[test]
    fn it_write_cursor_not_scale() {
        let cursor_color = PixelColor::blue();
        let border_color = PixelColor::yellow();
        let colors = CursorColors::new(cursor_color, border_color, Some(PixelColor::black()));

        let mut drawer = CursorDrawer::new(Vector2D::unit(), colors, PixelFormat::Rgb);
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
