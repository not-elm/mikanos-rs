use core::ops::Add;

use crate::math::size::Size;
use crate::math::vector::Vector2D;

impl<T> Add for Vector2D<T>
where
    T: Add + Copy,
{
    type Output = Vector2D<T::Output>;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}


impl<Num: Copy + Add<Output = Num>> Add<Num> for Vector2D<Num> {
    type Output = Vector2D<Num::Output>;

    fn add(self, rhs: Num) -> Self::Output {
        Vector2D {
            x: self.x() + rhs,
            y: self.y() + rhs,
        }
    }
}


impl Add<Size> for Vector2D<usize> {
    type Output = Vector2D<usize>;

    fn add(self, rhs: Size) -> Self::Output {
        Vector2D {
            x: self.x() + rhs.width(),
            y: self.y() + rhs.height(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::math::size::Size;
    use crate::math::vector::Vector2D;

    #[test]
    fn it_add_size() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 + Size::new(10, 10);

        assert_eq!(v2, Vector2D::new(15, 15));
    }


    #[test]
    fn it_add_scalar() {
        let v1 = Vector2D::new(5, 5);
        let v2 = v1 + 3;

        assert_eq!(v2, Vector2D::new(8, 8));
    }
}
