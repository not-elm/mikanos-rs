use core::ops::Mul;

use crate::math::size::Size;

impl Mul<Size> for Size {
    type Output = Size;

    fn mul(self, rhs: Size) -> Self::Output {
        Size::new(self.width() * rhs.width(), self.height() * rhs.height())
    }
}


#[cfg(test)]
mod tests {
    use crate::math::size::Size;

    #[test]
    fn it_mul() {
        let s1 = Size::new(10, 20);
        let s2 = Size::new(3, 5);
        assert_eq!(s1 * s2, Size::new(30, 100));
    }
}
