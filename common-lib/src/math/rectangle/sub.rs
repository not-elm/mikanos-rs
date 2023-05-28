use core::ops::Sub;

use crate::math::rectangle::Rectangle;
use crate::math::vector::Vector2D;

impl<Num: Copy + Sub<Output = Num>> Sub<Vector2D<Num>> for Rectangle<Num> {
    type Output = Rectangle<Num>;

    fn sub(self, rhs: Vector2D<Num>) -> Self::Output {
        Rectangle::new(self.origin - rhs, self.end - rhs)
    }
}


impl Rectangle<usize> {
    pub fn safe_sub_pos(&self, pos: &Vector2D<usize>) -> Rectangle<usize> {
        Rectangle::new(self.origin.safe_sub(pos), self.end.safe_sub(pos))
    }
}


#[cfg(test)]
mod tests {
    use crate::math::rectangle::Rectangle;
    use crate::math::vector::Vector2D;

    #[test]
    fn it_sub_vector2d() {
        let r = Rectangle::new(Vector2D::new(10, 10), Vector2D::new(30, 30));
        let pos = Vector2D::new(10, 10);
        assert_eq!(
            r - pos,
            Rectangle::new(Vector2D::zeros(), Vector2D::new(20, 20))
        );
    }


    #[test]
    fn it_safe_sub_pos() {
        let r = Rectangle::new(Vector2D::new(10, 10), Vector2D::new(30, 13));
        let pos = Vector2D::new(20, 15);
        assert_eq!(
            r.safe_sub_pos(&pos),
            Rectangle::new(Vector2D::zeros(), Vector2D::new(10, 0))
        );
    }
}
