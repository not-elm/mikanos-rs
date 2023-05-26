use core::fmt::Debug;

mod add;
mod add_assign;
mod mul;
mod mul_assign;
mod partial_eq;
mod sub;

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    x: T,
    y: T,
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


impl Vector2D<usize> {
    pub const fn unit() -> Self {
        Self::new(1, 1)
    }


    pub const fn zeros() -> Vector2D<usize> {
        Self::new(0, 0)
    }


    pub fn relative(&self, pos: Vector2D<usize>) -> Vector2D<isize> {
        let x = self.x() as isize - pos.x() as isize;
        let y = self.y() as isize - pos.y() as isize;

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
    use crate::math::vector::Vector2D;

    #[test]
    fn it_relative_move_left() {
        let origin = Vector2D::<usize>::new(5, 5);
        let moved = Vector2D::<usize>::new(3, 5);

        let relative = moved.relative(origin);

        assert_eq!(relative, Vector2D::new(-2, 0));
    }
}
