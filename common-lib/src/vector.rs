#[derive(Debug, Copy, Clone)]
pub struct Vector2D{
    x: isize,
    y: isize
}

impl Vector2D{
    pub fn new(x: isize, y: isize) -> Self{
        Self{
            x,
            y
        }
    }

    pub fn x(&self) -> isize{
        self.x
    }

    pub fn y(&self) -> isize{
        self.y
    }
}