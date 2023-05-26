use crate::math::vector::Vector2D;

impl<T: PartialEq> PartialEq for Vector2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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
