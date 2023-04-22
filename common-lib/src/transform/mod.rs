use crate::math::rectangle::Rectangle;
use crate::math::size::Size;
use crate::math::vector::Vector2D;

pub mod builder;

#[derive(Debug, Clone)]
pub struct Transform2D {
    size: Size,
    pos: Vector2D<usize>,
}


impl Transform2D {
    pub fn new(pos: Vector2D<usize>, size: Size) -> Self {
        Self { size, pos }
    }


    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
    }


    pub fn size(&self) -> Size {
        self.size
    }


    pub fn rect(&self) -> Rectangle<usize> {
        Rectangle::from_size(self.pos, self.size)
    }


    pub fn set_pos(&mut self, pos: Vector2D<usize>) {
        self.pos = pos;
    }

    pub fn with_in(&self, transform: &Transform2D) -> bool {
        self.rect()
            .with_in_rect(&transform.rect())
    }
}
