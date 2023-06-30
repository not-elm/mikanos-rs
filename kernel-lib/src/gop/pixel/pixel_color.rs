#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(C, packed)]
pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}


macro_rules! color {
    ($name: ident, r => $r: literal, g => $g: literal, b => $b: literal) => {
        #[inline(always)]
        pub const fn $name() -> Self {
            Self {
                r: $r,
                g: $g,
                b: $b,
            }
        }
    };

    ($name: ident, $color: literal) => {
        color!($name, r => $color, g => $color, b => $color);
    }
}


impl PixelColor {
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }


    color!(black, r => 0x00, g => 0x00, b => 0x00);
    color!(white, r => 0xFF, g => 0xFF, b => 0xFF);
    color!(red, r => 0xFF, g => 0x00, b => 0x00);
    color!(green, r => 0x00, g => 0xFF, b => 0x00);
    color!(blue, r => 0x00, g => 0x00, b => 0xFF);
    color!(yellow, r => 0xFF, g => 0xFF, b => 0x00);
    color!(window_background, 0xC6);


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
