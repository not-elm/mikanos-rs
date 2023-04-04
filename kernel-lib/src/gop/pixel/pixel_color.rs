#[derive(Debug, Copy, Clone)]
pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl PixelColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn black() -> Self {
        Self {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        }
    }
    pub fn white() -> Self {
        Self {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}
