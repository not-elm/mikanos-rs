use core::cmp::max;
use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub};

use crate::math::size::Size;

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}


impl<T> Add for Vector2D<T>
where
    T: Add + Copy,
{
    type Output = Vector2D<T::Output>;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}


impl<Num: Copy + Add<Output = Num>> Add<Num> for Vector2D<Num> {
    type Output = Vector2D<Num::Output>;

    fn add(self, rhs: Num) -> Self::Output {
        Vector2D {
            x: self.x() + rhs,
            y: self.y() + rhs,
        }
    }
}


impl Add<Size> for Vector2D<usize> {
    type Output = Vector2D<usize>;

    fn add(self, rhs: Size) -> Self::Output {
        Vector2D {
            x: self.x() + rhs.width(),
            y: self.y() + rhs.height(),
        }
    }
}


impl<Num: Copy + Sub<Output = Num>> Sub<Vector2D<Num>> for Vector2D<Num> {
    type Output = Vector2D<Num>;

    fn sub(self, rhs: Vector2D<Num>) -> Self::Output {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}


impl<Num: Copy + Sub<Output = Num>> Sub<Num> for Vector2D<Num> {
    type Output = Vector2D<Num>;

    fn sub(self, rhs: Num) -> Self::Output {
        Vector2D::new(self.x - rhs, self.y - rhs)
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


impl<Num: Copy + Mul<Output = Num>> Mul<Num> for Vector2D<Num> {
    type Output = Vector2D<Num>;

    fn mul(self, rhs: Num) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs)
    }
}


impl<Num: Copy + Mul<Output = Num>> MulAssign<Num> for Vector2D<Num> {
    fn mul_assign(&mut self, rhs: Num) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}


impl<T: Copy> Vector2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }


    pub const fn x(&self) -> T {
        self.x
    }


    pub const fn y(&self) -> T {
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


impl Vector2D<usize> {
    pub const fn unit() -> Self {
        Self::new(1, 1)
    }


    pub const fn zeros() -> Vector2D<usize> {
        Self::new(0, 0)
    }


    pub fn relative(&self, pos: Vector2D<usize>) -> Vector2D<isize> {
        let x = max(0, self.x() as isize - pos.x() as isize);
        let y = max(0, self.y() as isize - pos.y() as isize);

        Vector2D::new(x, y)
    }
}


impl Default for Vector2D<usize> {
    fn default() -> Self {
        Self::zeros()
    }
}


#[cfg(test)]
mod tests {
    use crate::math::size::Size;
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


    #[test]
    fn it_sub() {
        let v1 = Vector2D::new(5, 5);
        let v2 = Vector2D::new(20, 20);

        assert_eq!(v2 - v1, Vector2D::new(15, 15));
    }


    #[test]
    fn it_add_size() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 + Size::new(10, 10);

        assert_eq!(v2, Vector2D::new(15, 15));
    }


    #[test]
    fn it_add_scalar() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 + 3;

        assert_eq!(v2, Vector2D::new(8, 8));
    }


    #[test]
    fn it_sub_scalar() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 - 3;

        assert_eq!(v2, Vector2D::new(2, 2));
    }


    #[test]
    fn it_mul_scalar() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 * 3;

        assert_eq!(v2, Vector2D::new(15, 15));
    }


    #[test]
    fn it_mul_assign_scalar() {
        let mut v1 = Vector2D::new(5, 5);
        v1 *= 3;

        assert_eq!(v1, Vector2D::new(15, 15));
    }
}
