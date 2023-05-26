use core::ops::{Mul, MulAssign};

use crate::math::vector::Vector2D;

impl<Num: Copy + Mul<Output = Num>> MulAssign<Num> for Vector2D<Num> {
    fn mul_assign(&mut self, rhs: Num) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}


#[cfg(test)]
mod tests {
    use crate::math::vector::Vector2D;

    #[test]
    fn it_mul_assign_scalar() {
        let mut v1 = Vector2D::new(5, 5);
        v1 *= 3;

        assert_eq!(v1, Vector2D::new(15, 15));
    }
}
