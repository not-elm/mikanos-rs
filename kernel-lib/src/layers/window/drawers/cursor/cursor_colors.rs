use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Clone, Copy)]
pub struct CursorColors {
    foreground: PixelColor,
    border: PixelColor,
    transparent: Option<PixelColor>,
}


impl CursorColors {
    pub const fn new(
        foreground: PixelColor,
        border: PixelColor,
        transparent: Option<PixelColor>,
    ) -> Self {
        Self {
            foreground,
            border,
            transparent,
        }
    }


    pub fn foreground(&self) -> PixelColor {
        self.foreground
    }


    pub fn border(&self) -> PixelColor {
        self.border
    }


    pub fn transparent(&self) -> Option<PixelColor> {
        self.transparent
    }


    pub const fn change_border(self, border: PixelColor) -> Self {
        Self::new(self.foreground, border, self.transparent)
    }


    pub const fn change_foreground(self, foreground: PixelColor) -> Self {
        Self::new(foreground, self.border, self.transparent)
    }


    pub const fn change_transparent(self, transparent: PixelColor) -> Self {
        Self::new(self.foreground, self.border, Some(transparent))
    }


    pub const fn disabled_transparent(self) -> Self {
        Self::new(self.foreground, self.border, None)
    }
}


impl Default for CursorColors {
    fn default() -> Self {
        Self::new(PixelColor::white(), PixelColor::white(), None)
    }
}
