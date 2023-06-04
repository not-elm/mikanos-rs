mod sub;

use core::fmt::Debug;
use core::ops::{Add, Sub};

use crate::math::pixel_with_in_rect_iter::PointsWithInRectIter;
use crate::math::size::Size;
use crate::math::vector::{max_vector2d, min_vector2d, Vector2D};

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


    pub fn intersect(&self, r: &Rectangle<usize>) -> Option<Rectangle<usize>> {
        if self.end.x() < r.origin.x()
            || self.end.y() < r.origin.y()
            || r.end.x() < self.origin.x()
            || r.end.y() < self.origin.y()
        {
            return None;
        }


        let origin = max_vector2d(&self.origin, &r.origin);
        let end = min_vector2d(&self.end, &r.end);
        Some(Rectangle::new(origin, end))
    }


    pub fn union(&self, r: &Rectangle<usize>) -> Rectangle<usize> {
        let xs = [
            self.origin.x(),
            r.origin.x(),
            self.end.x(),
            r.end.x(),
        ]
        .into_iter();


        let ys = [
            self.origin.y(),
            r.origin.y(),
            self.end.y(),
            r.end.y(),
        ]
        .into_iter();


        let origin = Vector2D::new(xs.clone().min().unwrap(), ys.clone().min().unwrap());

        let end = Vector2D::new(xs.max().unwrap(), ys.max().unwrap());

        Rectangle::new(origin, end)
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


    pub fn overlap(&self, r2: &Rectangle<T>) -> bool {
        let x1 = self.origin().x();
        let w1 = self.end().x();
        let x2 = r2.origin().x();
        let w2 = r2.end().x();
        if !(x2 <= x1 && x1 <= w2 || x1 <= x2 && x2 <= w1) {
            return false;
        }

        let y1 = self.origin().y();
        let h1 = self.end().y();
        let y2 = r2.origin().y();
        let h2 = r2.end().y();

        y2 <= y1 && y1 <= h2 || y1 <= y2 && y2 <= h1
    }
}


impl Rectangle<usize> {
    pub fn width(&self) -> usize {
        self.end.x() - self.origin.x()
    }


    pub fn height(&self) -> usize {
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


impl<T: PartialEq + Copy + PartialOrd> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin() && self.end == other.end()
    }
}


#[cfg(test)]
mod tests {
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
    fn it_rect_add_size() {
        let rect = Rectangle::new(Vector2D::unit(), Vector2D::new(30usize, 30));
        let size = Size::new(30, 100);
        let rect = rect + size;

        assert_eq!(
            rect,
            Rectangle::new(Vector2D::unit(), Vector2D::new(60, 130))
        );
    }


    #[test]
    fn it_rect_overlap_if_inner_target() {
        let r1 = Rectangle::new(Vector2D::zeros(), Vector2D::new(30usize, 30));
        let r2 = Rectangle::new(Vector2D::unit(), Vector2D::new(30usize, 30));

        assert!(r1.overlap(&r2));
        assert!(r2.overlap(&r1));
    }


    #[test]
    fn it_rect_overlap_if_cross_sides() {
        let r1 = Rectangle::new(Vector2D::new(50, 50), Vector2D::new(100, 100));
        let r2 = Rectangle::new(Vector2D::new(100, 100), Vector2D::new(100, 100));

        assert!(r1.overlap(&r2));
        assert!(r2.overlap(&r1));
    }


    #[test]
    fn it_rect_not_overlap() {
        let r1 = Rectangle::new(Vector2D::new(00, 0), Vector2D::new(99, 99));
        let r2 = Rectangle::new(Vector2D::new(100, 100), Vector2D::new(100, 100));

        assert!(!r1.overlap(&r2));
        assert!(!r2.overlap(&r1));
    }


    #[test]
    fn it_rect_overlap_there_equal_size_and_pos() {
        let r1 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(99, 99));
        let r2 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(99, 99));

        assert!(r1.overlap(&r2));
        assert!(r2.overlap(&r1));
    }


    #[test]
    fn it_intersect_there_equal_size() {
        let r1 = Rectangle::new(Vector2D::new(0usize, 0), Vector2D::new(99, 99));
        let r2 = Rectangle::new(Vector2D::new(0usize, 0), Vector2D::new(99, 99));

        assert!(r1
            .intersect(&r2)
            .is_some_and(|r| r == r1));
        assert!(r2
            .intersect(&r1)
            .is_some_and(|r| r == r1));
    }


    #[test]
    fn it_intersect_when_cross_sides() {
        let r1 = Rectangle::new(
            Vector2D::new(100usize, 100usize),
            Vector2D::new(200usize, 200usize),
        );
        let r2 = Rectangle::new(Vector2D::new(10usize, 10), Vector2D::new(110, 110));
        println!("{:?}", r1.intersect(&r2));
        let rect = r1.intersect(&r2);
        assert!(rect.is_some_and(
            |r| r.origin() == Vector2D::new(100, 100) && r.end() == Vector2D::new(110, 110)
        ));

        let rect = r2.intersect(&r1);
        assert!(rect.is_some_and(
            |r| r.origin() == Vector2D::new(100, 100) && r.end() == Vector2D::new(110, 110)
        ));
    }


    #[test]
    fn it_intersect_3() {
        let r1 = Rectangle::new(Vector2D::new(0usize, 0), Vector2D::new(500, 300));
        let r2 = Rectangle::new(Vector2D::new(0usize, 0), Vector2D::new(1024, 768));

        assert!(r1
            .intersect(&r2)
            .is_some_and(|r| r == r1));
        assert!(r2
            .intersect(&r1)
            .is_some_and(|r| r == r1));
    }


    #[test]
    fn it_union_base_zeros() {
        let r1 = Rectangle::new(Vector2D::zeros(), Vector2D::new(10usize, 10usize));
        let r2 = Rectangle::new(Vector2D::zeros(), Vector2D::new(110, 110));

        let rect = r1.union(&r2);
        assert_eq!(rect.origin, Vector2D::zeros());
        assert_eq!(rect.end, Vector2D::new(110, 110));
    }


    #[test]
    fn it_union() {
        let r1 = Rectangle::new(Vector2D::new(0, 10), Vector2D::new(100usize, 110usize));
        let r2 = Rectangle::new(Vector2D::new(10, 0), Vector2D::new(70, 1330));

        let rect = r1.union(&r2);
        assert_eq!(rect.origin, Vector2D::zeros());
        assert_eq!(rect.end, Vector2D::new(100, 1330));
    }
}
