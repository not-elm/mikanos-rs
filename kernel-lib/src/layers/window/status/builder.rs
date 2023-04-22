use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::window::status::WindowStatus;

#[derive(Debug)]
pub struct WindowStatusBuilder {
    background: Option<PixelColor>,
    foreground: Option<PixelColor>,
    pos: Option<Vector2D<usize>>,
    size: Option<Size>,
}


impl WindowStatusBuilder {
    pub const fn new() -> Self {
        Self {
            background: None,
            foreground: None,
            pos: None,
            size: None,
        }
    }


    pub fn background(self, background_color: PixelColor) -> Self {
        Self {
            background: Some(background_color),
            foreground: self.foreground,
            pos: self.pos,
            size: self.size,
        }
    }


    pub fn foreground(self, color: PixelColor) -> Self {
        Self {
            background: self.background,
            foreground: Some(color),
            pos: self.pos,
            size: self.size,
        }
    }


    pub fn pos(self, pos: Vector2D<usize>) -> Self {
        Self {
            background: self.background,
            foreground: self.foreground,
            pos: Some(pos),
            size: self.size,
        }
    }


    pub fn size(self, size: Size) -> Self {
        Self {
            background: self.background,
            foreground: self.foreground,
            pos: self.pos,
            size: Some(size),
        }
    }


    pub fn build(self) -> WindowStatus {
        WindowStatus::new(
            self.background
                .unwrap_or(PixelColor::black()),
            self.foreground
                .unwrap_or(PixelColor::white()),
            self.pos
                .unwrap_or(Vector2D::default()),
            self.size
                .unwrap_or(Size::new(100, 100)),
        )
    }
}


impl Default for WindowStatusBuilder {
    fn default() -> Self {
        Self::new()
    }
}
