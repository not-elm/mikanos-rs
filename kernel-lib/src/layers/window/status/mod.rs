use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::gop::pixel::pixel_color::PixelColor;

pub mod builder;

pub struct WindowStatus {
    background_color: PixelColor,
    color: PixelColor,
    size: Size,
    pos: Vector2D<usize>,
}


impl WindowStatus {
    pub fn new(
        background_color: PixelColor,
        color: PixelColor,
        size: Size,
        pos: Vector2D<usize>,
    ) -> Self {
        Self {
            background_color,
            color,
            size,
            pos,
        }
    }
}
