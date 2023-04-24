use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone, Copy)]
pub struct CursorColors {
    foreground: PixelColor,
    border: PixelColor,
    background: PixelColor,
}


impl CursorColors {
    pub const fn new(foreground: PixelColor, border: PixelColor, background: PixelColor) -> CursorColors {
        Self {
            foreground,
            border,
            background,
        }
    }

    pub const fn change_foreground(self, foreground: PixelColor) -> CursorColors {
        Self::new(
            foreground,
            self.background,
            self.background,
        )
    }


    pub const fn change_border(self, border: PixelColor) -> Self {
        Self::new(
            self.foreground,
            border,
            self.background,
        )
    }


    pub const fn change_background(self, background: PixelColor) -> Self {
        Self::new(
            self.foreground,
            self.border,
            background,
        )
    }


    pub const fn foreground(&self) -> PixelColor{
        self.foreground
    }


    pub const fn border(&self) -> PixelColor{
        self.border
    }


    pub const fn background(&self) -> PixelColor{
        self.background
    }
}


impl Default for CursorColors {
    fn default() -> Self {
        Self::new(PixelColor::white(), PixelColor::black(), DISPLAY_BACKGROUND_COLOR)
    }
}