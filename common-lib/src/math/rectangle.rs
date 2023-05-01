use core::fmt::Debug;
use core::ops::{Add, Sub};

use crate::math::pixel_with_in_rect_iter::PointsWithInRectIter;
use crate::math::size::Size;
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


impl Rectangle<usize> {
    pub fn from_pos_and_size(pos: Vector2D<usize>, size: Size) -> Self {
        Self::new(
            pos,
            Vector2D::new(pos.x() + size.width(), pos.y() + size.height()),
        )
    }


    pub fn from_size(size: Size) -> Self {
        Self::from_pos_and_size(Vector2D::zeros(), size)
    }


    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }


    pub fn points(&self) -> PointsWithInRectIter {
        PointsWithInRectIter::new(self.origin, Size::new(self.width(), self.height()))
    }


    pub fn points_unbound(&self) -> PointsWithInRectIter {
        PointsWithInRectIter::new(self.origin, Size::new(self.width() - 1, self.height() - 1))
    }
}


impl<T: Copy + PartialOrd> Rectangle<T> {
    pub fn with_in_pos(&self, pos: &Vector2D<T>) -> bool {
        self.origin.x() <= pos.x()
            && self.origin.y() <= pos.y()
            && pos.x() <= self.end.x()
            && pos.y() <= self.end.y()
    }


    pub fn with_in_rect(&self, rect: &Rectangle<T>) -> bool {
        self.with_in_pos(&rect.origin) && self.with_in_pos(&rect.end)
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


impl Add<Size> for Rectangle<usize> {
    type Output = Rectangle<usize>;

    fn add(self, rhs: Size) -> Self::Output {
        Rectangle::new(self.origin, self.end + rhs)
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
    use alloc::vec::Vec;

    use crate::math::rectangle::Rectangle;
    use crate::math::size::Size;
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
        assert!(r.with_in_pos(&v));
    }


    #[test]
    fn it_less() {
        let r = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let v = Vector2D::new(-1, -1);
        assert!(!r.with_in_pos(&v));
    }


    #[test]
    fn it_over() {
        let r = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let v = Vector2D::new(111, 0);
        assert!(!r.with_in_pos(&v));
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


    #[test]
    fn it_from_size() {
        let pos = Vector2D::new(5, 10);
        let size = Size::new(5, 10);

        let rect = Rectangle::from_pos_and_size(pos, size);

        assert_eq!(rect.origin, pos);

        assert_eq!(rect.width(), 5);
        assert_eq!(rect.height(), 10);
        assert_eq!(rect.end, Vector2D::new(10, 20));
    }


    #[test]
    fn it_rect_with_in_parent() {
        let parent = Rectangle::new(Vector2D::zeros(), Vector2D::new(30, 30));
        let child = Rectangle::new(Vector2D::new(10, 10), Vector2D::new(20, 20));

        assert!(parent.with_in_rect(&child));
    }


    #[test]
    fn it_rect_with_in_parent_when_eq_size() {
        let parent = Rectangle::new(Vector2D::zeros(), Vector2D::new(30, 30));
        let child = Rectangle::new(Vector2D::zeros(), Vector2D::new(30, 30));

        assert!(parent.with_in_rect(&child));
    }


    #[test]
    fn it_rect_less_origin() {
        let parent = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30, 30));
        let child = Rectangle::new(Vector2D::new(-1, -1), Vector2D::new(30, 30));

        assert!(!parent.with_in_rect(&child));
    }


    #[test]
    fn it_rect_over_origin() {
        let parent = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30, 30));
        let child = Rectangle::new(Vector2D::new(40, 40), Vector2D::new(50, 50));

        assert!(!parent.with_in_rect(&child));
    }


    #[test]
    fn it_rect_less_end_than_parent_origin() {
        let parent = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30, 30));
        let child = Rectangle::new(Vector2D::new(-50, -50), Vector2D::new(-10, -10));

        assert!(!parent.with_in_rect(&child));
    }


    #[test]
    fn it_rect_over_origin_than_parent_end() {
        let parent = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30, 30));
        let child = Rectangle::new(Vector2D::new(50, 50), Vector2D::new(60, 60));

        assert!(!parent.with_in_rect(&child));
    }


    #[test]
    fn it_rect_points() {
        let rect = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30usize, 30));
        let points: Vec<Vector2D<usize>> = rect.points().collect();

        assert_eq!(points[0], rect.origin);
        assert_eq!(*points.last().unwrap(), rect.end);
    }


    #[test]
    fn it_rect_points_unbound() {
        let rect = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(30usize, 30));
        let points: Vec<Vector2D<usize>> = rect
            .points_unbound()
            .collect();

        assert_eq!(points[0], rect.origin);
        assert_eq!(*points.last().unwrap(), rect.end - 1);
    }


    #[test]
    fn it_rect_add_size() {
        let rect = Rectangle::new(Vector2D::unit(), Vector2D::new(30usize, 30));
        let size = Size::new(30, 100);
        let rect = rect + size;

        assert_eq!(
            rect,
            Rectangle::new(Vector2D::unit(), Vector2D::new(60, 130))
        );
    }
}
