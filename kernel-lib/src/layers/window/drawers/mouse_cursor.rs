use core::any::Any;

use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::WindowDrawable;
use common_lib::transform::Transform2D;

pub const CURSOR_WIDTH: usize = 15;

pub const CURSOR_HEIGHT: usize = 24;


const CURSOR_SHAPE: [&[u8; CURSOR_WIDTH]; CURSOR_HEIGHT] = [
    b"@              ",
    b"@@             ",
    b"@.@            ",
    b"@..@           ",
    b"@...@          ",
    b"@....@         ",
    b"@.....@        ",
    b"@......@       ",
    b"@.......@      ",
    b"@........@     ",
    b"@.........@    ",
    b"@..........@   ",
    b"@...........@  ",
    b"@............@ ",
    b"@......@@@@@@@@",
    b"@......@       ",
    b"@....@@.@      ",
    b"@...@ @.@      ",
    b"@..@   @.@     ",
    b"@.@    @.@     ",
    b"@@      @.@    ",
    b"@       @.@    ",
    b"         @.@   ",
    b"         @@@   ",
];


#[derive(Debug, Clone)]
pub struct MouseCursorDrawer {
    scale: Vector2D<usize>,
    color: PixelColor,
    border_color: PixelColor,
}


impl MouseCursorDrawer {
    pub fn new(scale: Vector2D<usize>, color: PixelColor, border_color: PixelColor) -> Self {
        Self {
            scale,
            color,
            border_color,
        }
    }

    pub fn set_color(&mut self, color: PixelColor) {
        self.color = color
    }

    pub fn set_border_color(&mut self, border_color: PixelColor) {
        self.border_color = border_color;
    }


    unsafe fn write_row(
        &mut self,
        transform: &Transform2D,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult {
        for y in 0..CURSOR_HEIGHT {
            for _ in 0..self.scale.y() {
                self.write_line(transform, y, writer)?;
            }
        }

        Ok(())
    }

    unsafe fn write_line(
        &mut self,
        transform: &Transform2D,
        y: usize,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult {
        for x in 0..CURSOR_WIDTH {
            for _ in 0..self.scale.x() {
                if let Some(color) = cursor_color_at(x, y, self.color, self.border_color) {
                    writer.write(x + transform.pos().x(), y + transform.pos().y(), &color)?;
                }
            }
        }

        Ok(())
    }
}


impl WindowDrawable for MouseCursorDrawer {
    fn draw(&mut self, transform: &Transform2D, writer: &mut dyn PixelWritable) -> KernelResult {
        unsafe { self.write_row(transform, writer) }
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Default for MouseCursorDrawer {
    fn default() -> Self {
        Self::new(
            Vector2D::new(1, 1),
            PixelColor::white(),
            PixelColor::black(),
        )
    }
}


fn cursor_color_at(
    x: usize,
    y: usize,
    cursor_color: PixelColor,
    border_color: PixelColor,
) -> Option<PixelColor> {
    let c = char::from(CURSOR_SHAPE[y][x]);
    if c == '@' {
        Some(border_color)
    } else if c == '.' {
        Some(cursor_color)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use common_lib::math::size::Size;

    use crate::gop::pixel::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::layers::window::drawers::mouse_cursor::{
        cursor_color_at, MouseCursorDrawer, CURSOR_HEIGHT, CURSOR_WIDTH,
    };
    use crate::layers::window::WindowDrawable;
    use common_lib::transform::builder::Transform2DBuilder;

    #[test]
    fn it_write_cursor_not_scale() {
        let mut drawer = MouseCursorDrawer::default();
        let mut writer = MockBufferPixelWriter::new(100, 100);
        assert!(drawer
            .draw(
                &Transform2DBuilder::new()
                    .size(Size::new(100, 100))
                    .build(),
                &mut writer,
            )
            .is_ok());
        for y in 0..CURSOR_HEIGHT {
            for x in 0..CURSOR_WIDTH {
                assert_eq!(
                    writer.pixel_at(x, y),
                    cursor_color_at(x, y, PixelColor::white(), PixelColor::black())
                        .unwrap_or(PixelColor::black())
                );
            }
        }
    }
}
