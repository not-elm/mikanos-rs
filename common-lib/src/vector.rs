use core::fmt::Debug;
use core::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}

impl<T> Add for Vector2D<T>
where
    T: Add + Copy + Clone + Debug,
{
    type Output = Vector2D<T::Output>;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}
impl<T> AddAssign for Vector2D<T>
where
    T: AddAssign + Copy + Clone + Debug,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: Copy + Clone + core::fmt::Debug> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}
