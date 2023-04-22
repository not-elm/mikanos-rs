use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::gop::pixel::pixel_color::PixelColor;

pub mod builder;

#[derive(Debug, Clone)]
pub struct WindowStatus {
    background: PixelColor,
    foreground: PixelColor,
    size: Size,
    pos: Vector2D<usize>,
}


impl WindowStatus {
    pub fn new(
        background: PixelColor,
        foreground: PixelColor,
        pos: Vector2D<usize>,
        size: Size,
    ) -> Self {
        Self {
            background,
            foreground,
            size,
            pos,
        }
    }


    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
    }


    pub fn size(&self) -> Size {
        self.size
    }


    pub fn window_rect(&self) -> Rectangle<usize> {
        Rectangle::from_size(self.pos, self.size)
    }


    pub fn background(&self) -> PixelColor {
        self.background
    }


    pub fn foreground(&self) -> PixelColor {
        self.foreground
    }


    pub fn set_pos(&mut self, pos: Vector2D<usize>){
        self.pos = pos;
    }
}
