use core::fmt::Debug;
use core::ops::{Add, Sub};

use crate::math::vector::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle<T: Copy> {
    origin: Vector2D<T>,
    end: Vector2D<T>,
}


impl<T: Copy> Rectangle<T> {
    pub fn new(origin: Vector2D<T>, end: Vector2D<T>) -> Rectangle<T> {
        Self { origin, end }
    }


    pub fn origin(&self) -> Vector2D<T> {
        self.origin
    }


    pub fn end(&self) -> Vector2D<T> {
        self.end
    }
}


impl<T: Copy + PartialOrd> Rectangle<T> {
    pub fn is_inner(&self, pos: Vector2D<T>) -> bool {
        self.origin.x() <= pos.x()
            && self.origin.y() <= pos.y()
            && pos.x() <= self.end.x()
            && pos.y() <= self.end.y()
    }
}


impl<T: Copy + Sub<Output = T>> Rectangle<T> {
    pub fn width(&self) -> T {
        self.end.x() - self.origin.x()
    }


    pub fn height(&self) -> T {
        self.end.y() - self.origin.y()
    }
}


impl<Num: Copy + Add<Output = Num>> Add<Vector2D<Num>> for Rectangle<Num> {
    type Output = Rectangle<Num>;

    fn add(self, rhs: Vector2D<Num>) -> Self::Output {
        Rectangle::new(self.origin + rhs, self.end + rhs)
    }
}


impl<Num: Copy + Sub<Output = Num>> Sub<Vector2D<Num>> for Rectangle<Num> {
    type Output = Rectangle<Num>;

    fn sub(self, rhs: Vector2D<Num>) -> Self::Output {
        Rectangle::new(self.origin - rhs, self.end - rhs)
    }
}


impl<T: PartialEq + Copy + PartialOrd> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin() && self.end == other.end()
    }
}


#[cfg(test)]
mod tests {
    use crate::math::rectangle::Rectangle;
    use crate::math::vector::Vector2D;

    #[test]
    fn it_partial_eq_rectangle() {
        let r1 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let r2 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        assert_eq!(r1, r2);
    }


    #[test]
    fn it_partial_ne_rectangle() {
        let r1 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(10, 100));
        let r2 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        assert_ne!(r1, r2);
    }


    #[test]
    fn it_inner() {
        let r = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let v = Vector2D::new(30, 30);
        assert!(r.is_inner(v));
    }


    #[test]
    fn it_less() {
        let r = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let v = Vector2D::new(-1, -1);
        assert!(!r.is_inner(v));
    }


    #[test]
    fn it_over() {
        let r = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let v = Vector2D::new(111, 0);
        assert!(!r.is_inner(v));
    }


    #[test]
    fn it_correct_width_when_origin_zeros() {
        let rect = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(10, 10));
        assert_eq!(rect.width(), 10);
    }


    #[test]
    fn it_correct_width_when_origin_10() {
        let rect = Rectangle::new(Vector2D::new(10, 10), Vector2D::new(100, 100));
        assert_eq!(rect.width(), 90);
    }


    #[test]
    fn it_correct_height_when_origin_zeros() {
        let rect = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(10, 10));
        assert_eq!(rect.height(), 10);
    }


    #[test]
    fn it_correct_height_when_origin_10() {
        let rect = Rectangle::new(Vector2D::new(10, 10), Vector2D::new(30, 30));
        assert_eq!(rect.height(), 20);
    }


    #[test]
    fn it_correct_height_when_() {
        let rect = Rectangle::new(Vector2D::new(10, 10), Vector2D::new(30, 30));

        assert_eq!(rect.height(), 20);
    }


    #[test]
    fn it_correct_add_position() {
        let rect = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30, 30));
        let moved_rect = rect + Vector2D::new(10, 10);

        assert_eq!(moved_rect.origin, Vector2D::new(10, 10));
        assert_eq!(moved_rect.end, Vector2D::new(40, 40));
    }


    #[test]
    fn it_correct_add_() {
        let rect = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30, 30));
        let moved_rect = rect + Vector2D::new(10, 10);

        assert_eq!(moved_rect.origin, Vector2D::new(10, 10));
        assert_eq!(moved_rect.end, Vector2D::new(40, 40));
    }
}
