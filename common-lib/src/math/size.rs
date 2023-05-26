use core::ops::{Div, Mul, MulAssign};

use crate::math::pixel_with_in_rect_iter::PointsWithInRectIter;
use crate::math::rectangle::Rectangle;
use crate::math::vector::Vector2D;

mod sub;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Size(Vector2D<usize>);


impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Self(Vector2D::new(width, height))
    }


    pub const fn width(&self) -> usize {
        self.0.x()
    }


    pub const fn height(&self) -> usize {
        self.0.y()
    }


    pub fn as_vec2d(&self) -> Vector2D<usize> {
        Vector2D::new(self.width(), self.height())
    }


    pub fn into_rect(self) -> Rectangle<usize> {
        Rectangle::from_size(self)
    }


    pub fn points(&self) -> PointsWithInRectIter {
        PointsWithInRectIter::new(
            Vector2D::zeros(),
            Size::new(self.width() - 1, self.height() - 1),
        )
    }
}


impl Mul<usize> for Size {
    type Output = Size;

    fn mul(self, rhs: usize) -> Self::Output {
        Size::new(self.width() * rhs, self.height() * rhs)
    }
}


impl MulAssign<usize> for Size {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}


impl Div for Size {
    type Output = Size;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.width() / rhs.width(), self.height() / rhs.height())
    }
}


#[cfg(test)]
mod tests {
    use crate::math::rectangle::Rectangle;
    use crate::math::size::Size;
    use crate::math::vector::Vector2D;

    #[test]
    fn it_new() {
        let size = Size::new(3, 5);
        assert_eq!(size.width(), 3);
        assert_eq!(size.height(), 5);
    }


    #[test]
    fn it_eq() {
        let s1 = Size::new(3, 5);
        let s2 = Size::new(3, 5);
        assert_eq!(s1, s2);
    }


    #[test]
    fn it_as_vec2d() {
        let v = Size::new(3, 5).as_vec2d();

        assert_eq!(v, Vector2D::new(3, 5));
    }


    #[test]
    fn it_into_rect() {
        let rect = Size::new(3, 5).into_rect();

        assert_eq!(rect, Rectangle::new(Vector2D::zeros(), Vector2D::new(3, 5)));
    }


    #[test]
    fn it_mul_assign() {
        let mut size = Size::new(100, 100);
        size *= 4;

        assert_eq!(size, Size::new(400, 400));
    }


    #[test]
    fn it_div() {
        let s1 = Size::new(100, 100);
        let s2 = Size::new(10, 10);

        assert_eq!(s1 / s2, Size::new(10, 10));
    }
}
