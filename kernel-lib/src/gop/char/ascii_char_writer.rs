#[derive(Default)]
pub struct AscIICharWriter {}

impl AscIICharWriter {
    pub fn new() -> Self {
        Self {}
    }
}

// impl CharWritable for AscIICharWriter {
//     fn write(&mut self, c: char, pos: Vector2D, color: &PixelColor, pixel_writer: &mut impl PixelWritable) {
//         let a: [u8; 16] = [
//             0b00000000,
//             0b00110000,
//             0b00110000,
//             0b00110000,
//             0b00110000,
//             0b01001000,
//             0b01001000,
//             0b01001000,
//             0b11111111,
//             0b10000001,
//             0b10000001,
//             0b10000001,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//             0b00000000,
//         ];
//         for dy in 0..16{
//             for dx in 0..8{
//                 if (a[dy] << dx) & 0x80u8{
//                 }
//             }
//         }
//     }
// }
