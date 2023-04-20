use crate::math::vector::Vector2D;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Size(Vector2D<usize>);


impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self(Vector2D::new(width, height))
    }

    pub fn width(&self) -> usize {
        self.0.x()
    }

    pub fn height(&self) -> usize {
        self.0.y()
    }
}


#[cfg(test)]
mod tests {
    use crate::math::size::Size;

    #[test]
    fn it_new() {
        let size = Size::new(3, 5);
        assert_eq!(size.width(), 3);
        assert_eq!(size.height(), 5);
    }


    #[test]
    fn it_eq() {
        let s1 = Size::new(3, 5);
        let s2 = Size::new(3, 5);
        assert_eq!(s1, s2);
    }
}