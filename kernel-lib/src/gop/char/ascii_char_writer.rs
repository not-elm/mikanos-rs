use crate::error::KernelError::NotSupportCharacter;
use crate::error::KernelResult;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::font::get_font_from;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use common_lib::vector::Vector2D;

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
        c: char,
        pos: Vector2D<usize>,
        color: &PixelColor,
        pixel_writer: &mut impl PixelWritable,
    ) -> KernelResult {
        let ascii_char = get_font_from(c).ok_or(NotSupportCharacter)?;
        let ascii_char = unsafe { core::slice::from_raw_parts_mut(ascii_char, 16) };
        for (dy, line) in ascii_char.iter().enumerate() {
            for dx in 0..8 {
                let is_need_write_bit = ((line << dx) & 0x80u8) != 0;
                if is_need_write_bit {
                    unsafe {
                        pixel_writer
                            .write(pos.x() + dx, pos.y() + dy, color)
                            .unwrap()
                    };
                }
            }
        }

        Ok(())
    }
}
