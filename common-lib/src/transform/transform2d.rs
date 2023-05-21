use auto_delegate::delegate;
use core::cmp::max;

use crate::math::rectangle::Rectangle;
use crate::math::size::Size;
use crate::math::vector::Vector2D;

#[delegate]
pub trait Transformable2D {
    fn move_to(&mut self, pos: Vector2D<usize>);


    fn resize(&mut self, size: Size);


    fn rect(&self) -> Rectangle<usize>;


    fn pos(&self) -> Vector2D<usize>;


    fn transform_ref(&self) -> &Transform2D;


    fn feed_transform(&mut self, transform: &Transform2D) {
        self.move_to(transform.pos());
        self.resize(transform.size());
    }


    fn move_to_relative(&mut self, pos: Vector2D<isize>) {
        let x = max(0, self.pos().x() as isize + pos.x()) as usize;
        let y = max(0, self.pos().y() as isize + pos.y()) as usize;

        self.move_to(Vector2D::new(x, y));
    }
}


#[derive(Debug, Clone)]
pub struct Transform2D {
    size: Size,
    pos: Vector2D<usize>,
}


impl Transform2D {
    pub fn new(pos: Vector2D<usize>, size: Size) -> Self {
        Self { size, pos }
    }


    pub fn size(&self) -> Size {
        self.size
    }


    pub fn rect(&self) -> Rectangle<usize> {
        Rectangle::from_pos_and_size(self.pos, self.size)
    }


    pub fn set_pos(&mut self, pos: Vector2D<usize>) {
        self.pos = pos;
    }


    pub fn resize(&mut self, size: Size) {
        self.size = size;
    }


    pub fn with_in(&self, transform: &Transform2D) -> bool {
        self.rect()
            .with_in_rect(&transform.rect())
    }
}


impl Transformable2D for Transform2D {
    fn move_to(&mut self, pos: Vector2D<usize>) {
        self.pos = pos;
    }


    fn resize(&mut self, size: Size) {
        self.size = size;
    }


    fn rect(&self) -> Rectangle<usize> {
        self.rect()
    }


    fn pos(&self) -> Vector2D<usize> {
        self.pos
    }


    fn transform_ref(&self) -> &Transform2D {
        self
    }
}
