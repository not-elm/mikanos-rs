use core::ops::Mul;

use crate::math::vector::Vector2D;

impl<Num: Copy + Mul<Output = Num>> Mul<Num> for Vector2D<Num> {
    type Output = Vector2D<Num>;

    fn mul(self, rhs: Num) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector::Vector2D;

    #[test]
    fn it_mul_scalar() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 * 3;

        assert_eq!(v2, Vector2D::new(15, 15));
    }
}
