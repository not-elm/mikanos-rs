use core::cmp::min;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::abs::abs;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::cursor::buffer::CursorBuffer;
use crate::layers::cursor::colors::CursorColors;

#[derive(Debug, Clone)]
pub struct CursorDrawer {
    cursor_buff: CursorBuffer,
    colors: CursorColors,
    pixel_writer: FrameBufferPixelWriter,
}


impl CursorDrawer {
    #[inline]
    pub fn new(config: FrameBufferConfig, scale: Vector2D<usize>, colors: CursorColors) -> Self {
        Self {
            cursor_buff: CursorBuffer::new(scale),
            colors,
            pixel_writer: FrameBufferPixelWriter::new(config),
        }
    }

    #[inline]
    pub fn new_use_default(config: FrameBufferConfig) -> Self {
        Self::new(config, Vector2D::unit(), CursorColors::default())
    }


    #[inline]
    pub fn cursor_size(&self) -> Size {
        self.cursor_buff.size()
    }


    #[inline]
    pub fn set_color(&mut self, colors: CursorColors) {
        self.colors = colors
    }


    pub fn update_back_buffer(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
        origin: Vector2D<usize>,
    ) -> KernelResult {
        let diff_y = abs(origin.y() as isize - draw_area.origin().y() as isize);
        let diff_x = abs(origin.x() as isize - draw_area.origin().x() as isize);
        let pixel_mapper = EnumPixelMapper::new(
            self.pixel_writer
                .config()
                .pixel_format,
        );

        for (y, mut row) in self
            .cursor_buff
            .pixel_frame(origin, self.colors, pixel_mapper)
            .enumerate()
            .skip_while(|(y, _)| diff_y != *y)
            .take_while(|(y, _)| origin.y() + y <= draw_area.end().y())
        {
            let pos = origin + Vector2D::new(diff_x, y);

            let origin = calc_pixel_pos(&self.pixel_writer.config(), pos.x(), pos.y())?;
            let line = row.pixels_buff(shadow_buff);
            let len = min(line.len() - diff_x * 4, draw_area.size().width() * 4);
            let end = origin + len;

            shadow_buff.raw_mut()[origin..end]
                .copy_from_slice(&line[diff_x * 4..(diff_x * 4 + len)]);
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
