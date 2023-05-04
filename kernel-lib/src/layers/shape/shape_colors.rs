use crate::gop::pixel::pixel_color::PixelColor;

#[derive(Debug, Copy, Clone)]
pub struct ShapeColors {
    foreground: PixelColor,
    transparent: Option<PixelColor>,
}


impl ShapeColors {
    pub const fn new(foreground: PixelColor, transparent: Option<PixelColor>) -> Self {
        Self {
            foreground,
            transparent,
        }
    }


    pub fn foreground(&self) -> PixelColor {
        self.foreground
    }


    pub fn foreground_ref(&self) -> &PixelColor {
        &self.foreground
    }


    pub fn transparent(&self) -> Option<PixelColor> {
        self.transparent
    }


    pub const fn change_foreground(self, foreground: PixelColor) -> Self {
        Self::new(foreground, self.transparent)
    }

    pub const fn change_transparent(self, transparent: PixelColor) -> Self {
        Self::new(self.foreground, Some(transparent))
    }


    pub const fn disable_transparent(self) -> Self {
        Self::new(self.foreground, None)
    }
}


impl Default for ShapeColors {
    fn default() -> Self {
        Self::new(PixelColor::white(), None)
    }
}