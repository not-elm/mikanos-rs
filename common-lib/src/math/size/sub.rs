use core::ops::Sub;

use crate::math::size::Size;

impl Sub<usize> for Size {
    type Output = Size;

    fn sub(self, rhs: usize) -> Self::Output {
        Size::new(self.width() - rhs, self.height() - rhs)
    }
}


impl Sub<Size> for Size {
    type Output = Option<Size>;

    fn sub(self, rhs: Size) -> Self::Output {
        let width = self
            .width()
            .checked_sub(rhs.width())?;

        let height = self
            .height()
            .checked_sub(rhs.height())?;

        Some(Size::new(width, height))
    }
}

#[cfg(test)]
mod tests {
    use crate::math::size::Size;

    #[test]
    fn it_sub_scalar() {
        let s = Size::new(100, 80);
        assert_eq!(s - 30, Size::new(70, 50))
    }


    #[test]
    fn it_sub_some() {
        let s = Size::new(100, 80);
        assert_eq!(s - Size::new(20, 30), Some(Size::new(80, 50)))
    }


    #[test]
    fn it_sub_non_over_width() {
        let s = Size::new(0, 110);
        assert_eq!(s - Size::new(20, 30), None)
    }


    #[test]
    fn it_sub_non_over_height() {
        let s = Size::new(30, 0);
        assert_eq!(s - Size::new(20, 30), None)
    }
}
