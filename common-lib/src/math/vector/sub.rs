use core::ops::Sub;

use crate::math::vector::Vector2D;

impl<Num: Copy + Sub<Output=Num>> Sub<Vector2D<Num>> for Vector2D<Num> {
    type Output = Vector2D<Num>;

    #[inline(always)]
    fn sub(self, rhs: Vector2D<Num>) -> Self::Output {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}


impl<Num: Copy + Sub<Output=Num>> Sub<Num> for Vector2D<Num> {
    type Output = Vector2D<Num>;

    fn sub(self, rhs: Num) -> Self::Output {
        Vector2D::new(self.x - rhs, self.y - rhs)
    }
}


impl Vector2D<usize> {
    pub fn safe_sub(&self, other: &Vector2D<usize>) -> Vector2D<usize> {
        Vector2D::new(
            self.x
                .saturating_sub(other.x()),
            self.y
                .saturating_sub(other.y()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector::Vector2D;

    #[test]
    fn it_sub() {
        let v1 = Vector2D::new(5, 5);
        let v2 = Vector2D::new(20, 20);

        assert_eq!(v2 - v1, Vector2D::new(15, 15));
    }


    #[test]
    fn it_sub_scalar() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 - 3;

        assert_eq!(v2, Vector2D::new(2, 2));
    }


    #[test]
    fn it_safe_sub() {
        let v1 = Vector2D::<usize>::new(3, 3);
        let v2 = Vector2D::<usize>::new(10, 10);

        assert_eq!(v1.safe_sub(&v2), Vector2D::zeros());
    }
}
