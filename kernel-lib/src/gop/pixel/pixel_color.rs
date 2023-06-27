#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(C, packed)]
pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl PixelColor {
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }


    #[inline(always)]
    pub const fn black() -> Self {
        Self {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        }
    }


    #[inline(always)]
    pub const fn white() -> Self {
        Self {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
        }
    }


    #[inline(always)]
    pub const fn yellow() -> Self {
        Self {
            r: 0xFF,
            g: 0xFF,
            b: 0x00,
        }
    }


    #[inline(always)]
    pub const fn blue() -> Self {
        Self {
            r: 0x00,
            g: 0x00,
            b: 0xFF,
        }
    }


    #[inline(always)]
    pub const fn window_background() -> Self {
        Self::new(0xC6, 0xC6, 0xC6)
    }


    #[inline(always)]
    pub const fn r(&self) -> u8 {
        self.r
    }


    #[inline(always)]
    pub const fn g(&self) -> u8 {
        self.g
    }


    #[inline(always)]
    pub const fn b(&self) -> u8 {
        self.b
    }
}
