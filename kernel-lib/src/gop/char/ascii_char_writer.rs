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
        c: char,
        pos: Vector2D,
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
        for dy in 0..16 {
            for dx in 0..8 {
                let v = (a[dy] << dx) & 0x80u8;
                if v != 0 {
                    unsafe { pixel_writer.write(dx, dy, color).unwrap() };
                }
            }
        }
    }
}
