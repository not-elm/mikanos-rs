use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelError::NotSupportCharacter;
use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::font::get_font_from;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::layers::text::colors::TextColors;

#[derive(Default)]
pub struct AscIICharWriter {}


impl AscIICharWriter {
    pub fn new() -> Self {
        Self {}
    }
}


impl CharWritable for AscIICharWriter {
    fn write(
        &mut self,
        dist_buff: &mut [u8],
        c: char,
        pos: Vector2D<usize>,
        colors: &TextColors,
        pixel_writer: &mut impl PixelWritable,
    ) -> KernelResult {
        if c == '\n' {
            return Ok(());
        }

        let ascii_char = get_font_from(c).ok_or(NotSupportCharacter)?;
        let ascii_char = unsafe { core::slice::from_raw_parts_mut(ascii_char, 16) };
        for (dy, line) in ascii_char.iter().enumerate() {
            for dx in 0..8 {
                let is_need_write_bit = ((line << dx) & 0x80u8) != 0;
                let pos = pos + Vector2D::new(dx, dy);
                let color = if is_need_write_bit {
                    colors.foreground()
                } else {
                    colors.background()
                };
                unsafe {
                    pixel_writer
                        .write(dist_buff, &pos, color)
                        .unwrap()
                };
            }
        }

        Ok(())
    }


    fn font_unit(&self) -> Size {
        Size::new(8, 16)
    }
}
