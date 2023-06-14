use core::num::TryFromIntError;

use auto_delegate::delegate;

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


    fn store_transform(&mut self, transform: &Transform2D) {
        self.move_to(transform.pos());
        self.resize(transform.size());
    }


    fn move_to_relative(&mut self, pos: Vector2D<isize>) -> Result<(), TryFromIntError> {
        let x = usize::try_from(self.pos().x() as isize + pos.x())?;

        let y = usize::try_from(self.pos().y() as isize + pos.y())?;

        self.move_to(Vector2D::new(x, y));

        Ok(())
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
