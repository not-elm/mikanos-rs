use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone)]
pub struct TextColors {
    foreground: PixelColor,
    background: PixelColor,
}

impl TextColors {
    pub const fn new(foreground: PixelColor, background: PixelColor) -> Self {
        Self {
            foreground,
            background,
        }
    }


    pub const fn change_foreground(self, foreground: PixelColor) -> Self {
        Self::new(foreground, self.background)
    }

    pub const fn change_background(self, background: PixelColor) -> Self {
        Self::new(self.foreground, background)
    }


    pub const fn foreground(&self) -> &PixelColor {
        &self.foreground
    }


    pub const fn background(&self) -> &PixelColor {
        &self.background
    }
}

impl Default for TextColors {
    fn default() -> Self {
        Self::new(PixelColor::yellow(), DISPLAY_BACKGROUND_COLOR)
    }
}
