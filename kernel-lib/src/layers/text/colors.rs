use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Copy, Clone)]
pub struct TextColors {
    foreground: PixelColor,
    background: PixelColor,
}


impl TextColors {
    #[inline]
    pub const fn new(foreground: PixelColor, background: PixelColor) -> Self {
        Self {
            foreground,
            background,
        }
    }


    #[inline(always)]
    pub const fn change_foreground(self, foreground: PixelColor) -> Self {
        Self::new(foreground, self.background)
    }


    #[inline(always)]
    pub const fn change_background(self, background: PixelColor) -> Self {
        Self::new(self.foreground, background)
    }


    #[inline(always)]
    pub const fn foreground(&self) -> PixelColor {
        self.foreground
    }


    #[inline(always)]
    pub const fn background(&self) -> PixelColor {
        self.background
    }


    #[inline(always)]
    pub const fn foreground_ref(&self) -> &PixelColor {
        &self.foreground
    }


    #[inline(always)]
    pub const fn background_ref(&self) -> &PixelColor {
        &self.background
    }
}

impl Default for TextColors {
    #[inline]
    fn default() -> Self {
        Self::new(PixelColor::yellow(), DISPLAY_BACKGROUND_COLOR)
    }
}
