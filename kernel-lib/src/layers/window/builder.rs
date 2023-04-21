use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::window::Window;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;


#[derive(Debug)]
pub struct WindowBuilder {
    background_color: Option<PixelColor>,
    color: Option<PixelColor>,
    pos: Option<Vector2D<usize>>,
    size: Option<Size>,
}


impl WindowBuilder {
    pub const fn new() -> Self {
        Self {
            background_color: None,
            color: None,
            pos: None,
            size: None,
        }
    }


    pub fn background_color(self, background_color: PixelColor) -> Self {
        Self {
            background_color: Some(background_color),
            color: self.color,
            pos: self.pos,
            size: self.size,
        }
    }


    pub fn color(self, color: PixelColor) -> Self {
        Self {
            background_color: self.background_color,
            color: Some(color),
            pos: self.pos,
            size: self.size,
        }
    }


    pub fn pos(self, pos: Vector2D<usize>) -> Self {
        Self {
            background_color: self.background_color,
            color: self.color,
            pos: Some(pos),
            size: self.size,
        }
    }


    pub fn size(self, size: Size) -> Self {
        Self {
            background_color: self.background_color,
            color: self.color,
            pos: self.pos,
            size: Some(size),
        }
    }


    pub fn build<Draw>(self, drawer: Draw) -> Window<Draw> {
        Window::new(
            drawer,
            self.background_color
                .unwrap_or(PixelColor::black()),
            self.color
                .unwrap_or(PixelColor::white()),
            self.pos
                .unwrap_or(Vector2D::default()),
            self.size
                .unwrap_or(Size::new(100, 100)),
        )
    }
}


impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
