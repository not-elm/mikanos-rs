use crate::gop::char::char_writable::CharWritable;
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
        _c: char,
        _pos: Vector2D,
        color: &PixelColor,
        pixel_writer: &mut impl PixelWritable,
    ) {
        let a: [u8; 16] = [
            0b00000000, 
            0b00110000,
            0b00110000,
            0b00110000,
            0b00110000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b11111111,
            0b10000001,
            0b10000001,
            0b10000001,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];
        for (dy, line) in a.iter().enumerate() {
            for dx in 0..8 {
                let is_need_write_bit = ((line << dx) & 0x80u8) != 0;
                if is_need_write_bit {
                    unsafe { pixel_writer.write(dx, dy, color).unwrap() };
                }
            }
        }
    }
}
