use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone, Copy)]
pub struct CursorColors {
    foreground: PixelColor,
    border: PixelColor,
}


impl CursorColors {
    #[inline(always)]
    pub const fn new(foreground: PixelColor, border: PixelColor) -> Self {
        Self { foreground, border }
    }


    #[inline(always)]
    pub fn foreground(&self) -> PixelColor {
        self.foreground
    }


    #[inline(always)]
    pub fn border(&self) -> PixelColor {
        self.border
    }


    #[inline(always)]
    pub const fn change_border(self, border: PixelColor) -> Self {
        Self::new(self.foreground, border)
    }


    #[inline(always)]
    pub const fn change_foreground(self, foreground: PixelColor) -> Self {
        Self::new(foreground, self.border)
    }
}


impl Default for CursorColors {
    #[inline(always)]
    fn default() -> Self {
        Self::new(PixelColor::white(), PixelColor::white())
    }
}
