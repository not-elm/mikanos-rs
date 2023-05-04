use crate::math::rectangle::Rectangle;
use crate::math::size::Size;
use crate::math::vector::Vector2D;

pub trait Transformable2D {
    fn move_to(&mut self, pos: Vector2D<usize>);

    fn resize(&mut self, size: Size);


    fn rect(&self) -> Rectangle<usize>;
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


    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
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
}


#[macro_export]
macro_rules! impl_transformable2D {
    ($struct_name: ident) => {
        impl common_lib::transform::transform2d::Transformable2D for $struct_name {
            fn move_to(&mut self, pos: common_lib::math::vector::Vector2D<usize>) {
                self.transform.set_pos(pos);
            }


            fn resize(&mut self, size: common_lib::math::size::Size) {
                self.transform.resize(size);
            }


            fn rect(&self) -> common_lib::math::rectangle::Rectangle<usize> {
                self.transform.rect()
            }
        }
    };
}
