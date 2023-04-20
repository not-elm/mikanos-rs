use crate::math::vector::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle<T: Copy + PartialOrd> {
    origin: Vector2D<T>,
    end: Vector2D<T>,
}


impl<T: Copy + PartialOrd> Rectangle<T> {
    pub fn new(origin: Vector2D<T>, end: Vector2D<T>) -> Rectangle<T> {
        Self { origin, end }
    }


    pub fn origin(&self) -> Vector2D<T> {
        self.origin
    }


    pub fn end(&self) -> Vector2D<T> {
        self.end
    }

    pub fn is_inner(&self, pos: Vector2D<T>) -> bool {
        self.origin.x() <= pos.x()
            && self.origin.y() <= pos.y()
            && pos.x() <= self.end.x()
            && pos.y() <= self.end.y()
    }
}


impl<T: PartialEq + Copy + PartialOrd> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin() && self.end == other.end()
    }
}


#[cfg(test)]
mod tests {
    use crate::math::vector::Vector2D;
    use crate::math::rectangle::Rectangle;

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
}
