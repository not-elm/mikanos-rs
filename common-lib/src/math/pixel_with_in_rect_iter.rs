use crate::math::size::Size;
use crate::math::vector::Vector2D;

#[derive(Debug, Clone)]
pub struct PointsWithInRectIter {
    origin: Vector2D<usize>,
    size: Size,
    x: usize,
    y: usize,
}


impl PointsWithInRectIter {
    pub const fn new(origin: Vector2D<usize>, size: Size) -> Self {
        Self {
            origin,
            size,
            x: origin.x(),
            y: origin.y(),
        }
    }
}


impl Iterator for PointsWithInRectIter {
    type Item = Vector2D<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let limit = self.size;
        if limit.height() + self.origin.y() < self.y {
            return None;
        }

        if limit.width() + self.origin.x() < self.x {
            self.y += 1;
            self.x = self.origin.x();
            self.next()
        } else {
            let v = Vector2D::new(self.x, self.y);
            self.x += 1;
            Some(v)
        }
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::math::pixel_with_in_rect_iter::PointsWithInRectIter;
    use crate::math::size::Size;
    use crate::math::vector::Vector2D;

    #[test]
    fn it_collect_range() {
        let vec2d_buff: Vec<Vector2D<usize>> =
            PointsWithInRectIter::new(Vector2D::zeros(), Size::new(100, 100)).collect();


        for y in 0..=100 {
            for x in 0..=100 {
                assert_eq!(vec2d_buff[x + (y * 101)], Vector2D::new(x, y));
            }
        }
    }


    #[test]
    fn it_collect_range_with_origin() {
        let vec2d_buff: Vec<Vector2D<usize>> =
            PointsWithInRectIter::new(Vector2D::unit(), Size::new(100, 100)).collect();


        for y in 1..=100 {
            for x in 1..=100 {
                assert_eq!(vec2d_buff[(x - 1) + ((y - 1) * 100)], Vector2D::new(x, y));
            }
        }
    }
}
