use core::fmt::Debug;
use core::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}

impl<T> Add for Vector2D<T>
where
    T: Add + Copy + Clone + Debug + PartialOrd + PartialEq,
{
    type Output = Vector2D<T::Output>;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl<T: Copy + PartialOrd> AddAssign for Vector2D<T>
where
    T: AddAssign + Copy + Clone + Debug,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + PartialOrd> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }


    pub fn x(&self) -> T {
        self.x
    }


    pub fn y(&self) -> T {
        self.y
    }
}


impl<T: PartialOrd> Vector2D<T> {
    pub fn is_over_x(&self, other: &Vector2D<T>) -> bool {
        self.x < other.x
    }


    pub fn is_over_y(&self, other: &Vector2D<T>) -> bool {
        self.y < other.y
    }

    pub fn is_over(&self, other: &Vector2D<T>) -> bool {
        self.is_over_x(other) || self.is_over_y(other)
    }
}


impl<T: PartialEq> PartialEq for Vector2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl Default for Vector2D<usize> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}


#[cfg(test)]
mod tests {
    use crate::math::vector::Vector2D;

    #[test]
    fn it_partial_eq() {
        let v1 = Vector2D::new(11, 13);
        let v2 = Vector2D::new(11, 13);
        assert_eq!(v1, v2);
    }


    #[test]
    fn it_partial_ne() {
        let v1 = Vector2D::new(10, 13);
        let v2 = Vector2D::new(11, 13);
        assert_ne!(v1, v2);
    }
}
