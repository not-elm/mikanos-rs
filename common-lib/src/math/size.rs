use crate::math::pixel_with_in_rect_iter::PointsWithInRectIter;
use crate::math::rectangle::Rectangle;
use crate::math::vector::Vector2D;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Size(Vector2D<usize>);


impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self(Vector2D::new(width, height))
    }


    pub fn width(&self) -> usize {
        self.0.x()
    }


    pub fn height(&self) -> usize {
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
}
