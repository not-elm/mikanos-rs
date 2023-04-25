use core::any::Any;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;
use crate::layers::window::drawers::cursor::cursor_buffer::CursorBuffer;
use crate::layers::window::drawers::cursor::cursor_colors::CursorColors;
use crate::layers::window::WindowDrawable;

#[derive(Debug, Clone)]
pub struct MouseCursorDrawer {
    cursor_buff: CursorBuffer,
    colors: CursorColors,
    converter: EnumPixelConverter,
}


impl MouseCursorDrawer {
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


impl WindowDrawable for MouseCursorDrawer {
    fn draw_in_area(
        &mut self,
        _window_transform: &Transform2D,
        draw_rect: &Rectangle<usize>,
        writer: &mut dyn PixelFlushable,
    ) -> KernelResult {
        let pixel_frame =
            self.cursor_buff
                .pixel_frame(draw_rect.origin(), self.colors, self.converter.clone());


        unsafe { writer.flush(pixel_frame) }
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for MouseCursorDrawer {
    fn default() -> Self {
        Self::new(Vector2D::unit(), CursorColors::default(), PixelFormat::Rgb)
    }
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn it_write_cursor_not_scale() {
    //     let cursor_color = PixelColor::blue();
    //     let border_color = PixelColor::yellow();
    //     let mut drawer = MouseCursorDrawer::new(Vector2D::unit(),
    // cursor_color, border_color);     let mut writer =
    // MockBufferPixelWriter::new(         drawer.cursor_size().width() * 4,
    //         drawer.cursor_size().height() * 4,
    //     );
    //
    //     let transform = Transform2DBuilder::new()
    //         .size(Size::new(
    //             drawer.cursor_size().width() * 4,
    //             drawer.cursor_size().height() * 4,
    //         ))
    //         .build();
    //
    //     assert!(drawer
    //         .draw(&transform, &mut writer)
    //         .is_ok());
    //
    //     let pixels = drawer
    //         .cursor_buff
    //         .cursor_pixels_checked(&transform, cursor_color, border_color)
    //         .unwrap();
    //
    //     let points: Vec<Vector2D<usize>> = drawer
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
