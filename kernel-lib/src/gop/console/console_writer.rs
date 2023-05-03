use core::fmt::Error;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::gop::pixel::{fill_rect, select_pixel_writer};

type ImplCharWritable = impl CharWritable;
pub type ImplPixelWritable = impl PixelWritable;

/// コンソールの縦幅(MikanOSに合わせています。)
const HEIGHT: usize = 26;

/// コンソールの横幅(MikanOSに合わせています)
const WIDTH: usize = 81;


pub struct ConsoleWriter {
    char_writer: ImplCharWritable,
    pixel_writer: ImplPixelWritable,
    y: usize,
    x: usize,
    color: PixelColor,
    chars: [[char; WIDTH]; HEIGHT],
}

impl core::fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_str(s) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::default()),
        }
    }
}

impl ConsoleWriter {
    pub(crate) fn new(frame_buffer_config: FrameBufferConfig, color: PixelColor) -> Self {
        Self {
            char_writer: crate::gop::char::new_char_writer(),
            pixel_writer: select_pixel_writer(frame_buffer_config),
            y: 0,
            x: 0,

            color,
            chars: [[char::default(); WIDTH]; HEIGHT],
        }
    }

    pub fn pixel_writer(&mut self) -> &mut ImplPixelWritable {
        &mut self.pixel_writer
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn write_pixel(&mut self, pos: Vector2D<usize>, color: PixelColor) -> KernelResult {
        unsafe {
            self.pixel_writer
                .write(pos.x(), pos.y(), &color)
        }
    }
    pub fn write_str(&mut self, s: &str) -> KernelResult {
        for c in s.chars() {
            self.next_write_char(c)?;
        }

        Ok(())
    }

    fn next_write_char(&mut self, c: char) -> KernelResult {
        if c == '\n' || c == char::default() {
            return self.new_line();
        }

        if self.x >= self.max_x() {
            self.new_line()?;
        }

        self.write_char(c, Vector2D::new(self.x, self.y))?;
        self.x += 1;
        Ok(())
    }

    fn write_char(&mut self, c: char, pos: Vector2D<usize>) -> KernelResult {
        let write_pos = Vector2D::new(pos.x() * 8, pos.y() * 16);
        self.char_writer
            .write(c, write_pos, &(self.color), &mut self.pixel_writer)?;

        self.chars[pos.y()][pos.x()] = c;
        Ok(())
    }

    fn new_line(&mut self) -> KernelResult {
        if self.y < self.max_y() {
            self.y += 1;
            self.x = 0;
        } else {
            self.up_shift_lines()?;
        }

        Ok(())
    }

    fn chart_at(&self, pos: Vector2D<usize>) -> char {
        self.chars[pos.y()][pos.x()]
    }


    fn up_shift_lines(&mut self) -> KernelResult {
        // self.clear_display()?;
        self.shift_chars();
        self.flush()?;

        self.y = self.chars.len() - 1;
        self.x = 0;

        Ok(())
    }


    fn shift_chars(&mut self) {
        for y in 1..=self.max_y() {
            for x in 0..=self.max_x() {
                self.chars[y - 1][x] = self.chart_at(Vector2D::new(x, y));
            }
        }

        let end_line = self.chars.last_mut().unwrap();
        end_line.fill(char::default());
    }


    fn flush(&mut self) -> KernelResult {
        for y in 0..=self.max_y() {
            self.clear_line(y)?;
            for x in 0..=self.max_x() {
                let c = self.chart_at(Vector2D::new(x, y));
                if c == char::default() || c == '\n' {
                    break;
                }
                self.write_char(c, Vector2D::new(x, y))?;
            }
        }
        Ok(())
    }


    fn clear_line(&mut self, y: usize) -> KernelResult {
        let to = Vector2D::new(self.max_x() * 8, (y * 16) + 16);
        fill_rect(
            &mut self.pixel_writer,
            Vector2D::new(0, y * 16),
            to,
            DISPLAY_BACKGROUND_COLOR,
        )
    }

    fn max_y(&self) -> usize {
        self.chars.len() - 1
    }


    fn max_x(&self) -> usize {
        self.chars[0].len() - 1
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::math::vector::Vector2D;

    use crate::gop::console::console_builder::ConsoleBuilder;
    use crate::gop::console::console_writer::HEIGHT;

    #[test]
    fn it_new_line() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str("\n")
            .is_ok());
        assert_eq!(console.y(), 1)
    }

    #[test]
    fn it_not_new_line() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str("test")
            .is_ok());
        assert_eq!(console.y(), 0)
    }

    #[test]
    fn it_over_column() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str(
                "012345678901234567890123456789012345678901234567890123456789012345678901234567890"
            )
            .is_ok());
        assert_eq!(console.y(), 1);
        assert_eq!(console.x(), 1);
    }

    #[test]
    fn it_get_0() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str(
                "012345678901234567890123456789012345678901234567890123456789012345678901234567890"
            )
            .is_ok());
        assert_eq!(console.y(), 1);
        assert_eq!(console.chart_at(Vector2D::new(0, 1)), '0');
    }

    #[test]
    fn it_scroll_display() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        for i in 0..HEIGHT {
            assert!(console
                .write_str(&format!("{}\n", i))
                .is_ok());
        }

        assert_eq!(console.chart_at(Vector2D::new(0, 0)), '1');
    }

    #[test]
    fn it_end_when_scroll_display() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        for i in 0..HEIGHT {
            assert!(console
                .write_str(&format!("{}\n", i))
                .is_ok());
        }

        assert_eq!(console.chart_at(Vector2D::new(1, 0)), '\0');
        assert_eq!(console.chart_at(Vector2D::new(2, HEIGHT - 1)), '\0');
    }

    #[test]
    fn it_two_new_line() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str("test\n")
            .is_ok());
        assert!(console
            .write_str("test\n")
            .is_ok());
        assert_eq!(console.y(), 2);
        assert_eq!(console.x(), 0);
    }
}
