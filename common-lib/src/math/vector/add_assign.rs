use core::fmt::Debug;
use core::ops::AddAssign;

use crate::math::vector::Vector2D;

impl<T: Copy + PartialOrd> AddAssign for Vector2D<T>
where
    T: AddAssign + Copy + Clone + Debug,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector::Vector2D;

    #[test]
    fn it_add_assign() {
        let mut v = Vector2D::new(0, 0);
        v += Vector2D::new(10, 30);
        assert_eq!(v, Vector2D::new(10, 30))
    }


    #[test]
    fn it_add_assign_minus() {
        let mut v = Vector2D::new(0, 0);
        v += Vector2D::new(-10, -30);
        assert_eq!(v, Vector2D::new(-10, -30))
    }
}
