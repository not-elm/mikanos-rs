use crate::math::size::Size;
use crate::math::vector::Vector2D;
use crate::transform::Transform2D;


#[derive(Debug)]
pub struct Transform2DBuilder {
    pos: Option<Vector2D<usize>>,
    size: Option<Size>,
}


impl Transform2DBuilder {
    pub const fn new() -> Self {
        Self {
            pos: None,
            size: None,
        }
    }


    pub fn pos(self, pos: Vector2D<usize>) -> Self {
        Self {
            pos: Some(pos),
            size: self.size,
        }
    }


    pub fn size(self, size: Size) -> Self {
        Self {
            pos: self.pos,
            size: Some(size),
        }
    }


    pub fn build(self) -> Transform2D {
        Transform2D::new(
            self.pos
                .unwrap_or(Vector2D::zeros()),
            self.size
                .unwrap_or(Size::new(100, 100)),
        )
    }
}


impl Default for Transform2DBuilder {
    fn default() -> Self {
        Self::new()
    }
}
