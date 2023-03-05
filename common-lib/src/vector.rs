#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    x: T,
    y: T,
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
