use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::cursor::cursor_buffer::CursorBuffer;
use crate::layers::cursor::cursor_colors::CursorColors;
use crate::layers::layer_updatable::LayerUpdatable;

#[derive(Debug, Clone)]
pub struct CursorDrawer {
    cursor_buff: CursorBuffer,
    colors: CursorColors,
    pixel_writer: FrameBufferPixelWriter,
}


impl CursorDrawer {
    pub fn new(config: FrameBufferConfig, scale: Vector2D<usize>, colors: CursorColors) -> Self {
        Self {
            cursor_buff: CursorBuffer::new(scale),
            colors,
            pixel_writer: FrameBufferPixelWriter::new(config),
        }
    }


    pub fn new_use_default(config: FrameBufferConfig) -> Self {
        Self::new(config, Vector2D::unit(), CursorColors::default())
    }


    pub fn cursor_size(&self) -> Size {
        self.cursor_buff.size()
    }


    pub fn set_color(&mut self, colors: CursorColors) {
        self.colors = colors
    }
}


impl LayerUpdatable for CursorDrawer {
    fn update_back_buffer(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for pixel in self
            .cursor_buff
            .cursor_pixels(draw_area.origin(), Some(draw_area.end()), self.colors)
        {
            if let Some(color) = pixel.color() {
                unsafe {
                    self.pixel_writer
                        .write(shadow_buff.raw_mut(), &pixel.pos(), &color)?;
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn it_write_cursor_not_scale() {
    //     let cursor_color = PixelColor::blue();
    //     let border_color = PixelColor::yellow();
    //     let colors = CursorColors::new(cursor_color, border_color,
    // Some(PixelColor::black()));
    //
    //     let mut layer_updatable = CursorDrawer::new(Vector2D::unit(),
    // colors);     let mut writer = FrameBufferPixelWriter::new(
    //         layer_updatable.cursor_size().width() * 4,
    //         layer_updatable.cursor_size().height() * 4,
    //     );
    //
    //     let transform = Transform2DBuilder::new()
    //         .size(Size::new(
    //             layer_updatable.cursor_size().width() * 4,
    //             layer_updatable.cursor_size().height() * 4,
    //         ))
    //         .build();
    //
    //     assert!(layer_updatable
    //         .draw(&transform, &mut writer)
    //         .is_ok());
    //
    //     let pixels = layer_updatable
    //         .cursor_buff
    //         .cursor_pixels_checked(&transform, cursor_color, border_color)
    //         .unwrap();
    //
    //     let points: Vec<Vector2D<usize>> = layer_updatable
    //         .cursor_size()
    //         .points()
    //         .collect();
    //
    //     points
    //         .into_iter()
    //         .zip(pixels)
    //         .for_each(|(point, cursor_pixel)| {
    //             let actual = writer.pixel_at(point.x(), point.y());
    //
    //             let expect = cursor_pixel
    //                 .color()
    //                 .unwrap_or(PixelColor::black());
    //
    //             assert_eq!(actual, expect);
    //         });
    // }
}
