use core::fmt::Error;

use crate::error::KernelResult;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;

use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::gop::pixel::{fill_rect, select_pixel_writer};

type ImplCharWritable = impl CharWritable;
pub type ImplPixelWritable = impl PixelWritable;
/// コンソールの縦幅(MikanOSに合わせています。)
const CONSOLE_ROW: usize = 25;

/// コンソールの横幅(MikanOSに合わせています)
const CONSOLE_COLUMN: usize = 80;

pub struct ConsoleWriter {
    char_writer: ImplCharWritable,
    pixel_writer: ImplPixelWritable,
    current_row: usize,
    current_column: usize,
    color: PixelColor,
    chars: [[char; CONSOLE_COLUMN + 1]; CONSOLE_ROW],
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
            current_row: 0,
            current_column: 0,

            color,
            chars: [[char::default(); CONSOLE_COLUMN + 1]; CONSOLE_ROW],
        }
    }

    pub fn current_row(&self) -> usize {
        self.current_row
    }

    pub fn current_column(&self) -> usize {
        self.current_column
    }
    pub fn write_pixel(&mut self, pos: Vector2D<usize>, color: PixelColor) -> KernelResult {
        unsafe { self.pixel_writer.write(pos.x(), pos.y(), &color) }
    }
    pub fn write_str(&mut self, s: &str) -> KernelResult {
        for c in s.chars() {
            self.next_write_char(c)?;
        }

        Ok(())
    }

    fn next_write_char(&mut self, c: char) -> KernelResult {
        if c == '\n' {
            return self.new_line();
        }

        if self.current_column >= CONSOLE_COLUMN || self.current_row() >= CONSOLE_ROW {
            self.new_line()?;
        }
        self.write_char(c, Vector2D::new(self.current_column(), self.current_row()))?;
        self.current_column += 1;
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
        if self.current_row < CONSOLE_ROW {
            self.chars[self.current_row][self.current_column] = '\n';
            self.current_row += 1;
            self.current_column = 0;
        } else {
            self.chars[self.current_row - 1][self.current_column] = '\n';
            self.up_shift_lines()?;
        }

        Ok(())
    }

    #[cfg(test)]
    fn chart_at(&self, pos: Vector2D<usize>) -> char {
        self.chars[pos.y()][pos.x()]
    }
    fn up_shift_lines(&mut self) -> KernelResult {
        self.clear_display()?;

        for y in 1..CONSOLE_ROW {
            for x in 0..=CONSOLE_COLUMN {
                let c = self.chars[y][x];
                self.chars[y - 1][x] = c;
                if c == '\n' {
                    break;
                }
                self.write_char(c, Vector2D::new(x, y - 1))?;
            }
        }
        self.current_row = CONSOLE_ROW - 1;
        self.current_column = 0;

        let end_line = self.chars.last_mut().unwrap();
        end_line.fill(char::default());

        Ok(())
    }

    fn clear_display(&mut self) -> KernelResult {
        fill_rect(
            &mut self.pixel_writer,
            Vector2D::new(0, 0),
            Vector2D::new(CONSOLE_COLUMN * 8, CONSOLE_ROW * 16),
            &PixelColor::new(0x00, 0x00, 0x00),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::gop::console::console_builder::ConsoleBuilder;
    use crate::gop::console::console_writer::CONSOLE_ROW;
    use alloc::format;
    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::vector::Vector2D;

    #[test]
    fn it_new_line() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console.write_str("\n").is_ok());
        assert_eq!(console.current_row(), 1)
    }

    #[test]
    fn it_not_new_line() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console.write_str("test").is_ok());
        assert_eq!(console.current_row(), 0)
    }

    #[test]
    fn it_over_column() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str(
                "012345678901234567890123456789012345678901234567890123456789012345678901234567890"
            )
            .is_ok());
        assert_eq!(console.current_row(), 1);
        assert_eq!(console.current_column(), 1);
    }
    #[test]
    fn it_get_0() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console
            .write_str(
                "012345678901234567890123456789012345678901234567890123456789012345678901234567890"
            )
            .is_ok());
        assert_eq!(console.current_row(), 1);
        assert_eq!(console.chart_at(Vector2D::new(0, 1)), '0');
    }
    #[test]
    fn it_scroll_display() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        for i in 0..=CONSOLE_ROW {
            assert!(console.write_str(&format!("{}\n", i)).is_ok());
        }

        assert_eq!(console.chart_at(Vector2D::new(0, 0)), '1');
    }
    #[test]
    fn it_end_when_scroll_display() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        for i in 0..=CONSOLE_ROW {
            assert!(console.write_str(&format!("{}\n", i)).is_ok());
        }

        assert_eq!(console.chart_at(Vector2D::new(1, 0)), '\n');
        assert_eq!(console.chart_at(Vector2D::new(2, 24)), '\n');
    }
    #[test]
    fn it_two_new_line() {
        let mut console = ConsoleBuilder::new().build(FrameBufferConfig::mock());
        assert!(console.write_str("test\n").is_ok());
        assert!(console.write_str("test\n").is_ok());
        assert_eq!(console.current_row(), 2);
        assert_eq!(console.current_column(), 0);
    }
}
