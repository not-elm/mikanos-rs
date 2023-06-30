use core::ops::Div;

use crate::math::size::Size;

impl Div for Size {
    type Output = Size;


    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.width() / rhs.width(), self.height() / rhs.height())
    }
}


impl Div<usize> for Size {
    type Output = Size;

    fn div(self, rhs: usize) -> Self::Output {
        Size::new(self.width() / rhs, self.height() / rhs)
    }
}


#[cfg(test)]
mod tests {
    use crate::math::size::Size;

    #[test]
    fn it_div_scalar() {
        let size = Size::new(100, 100);
        assert_eq!(size / 4, Size::new(25, 25));
    }
}
