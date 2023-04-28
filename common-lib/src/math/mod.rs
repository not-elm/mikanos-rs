use core::usize;

pub mod vector;
pub mod size;
pub mod unit;
pub mod rectangle;
pub mod pixel_with_in_rect_iter;
pub mod numeric;

pub fn frame_count_from_bytes(bytes: usize, frame_size: usize) -> usize {
    if frame_size == 0 {
        return 1;
    }

    let diff = bytes % frame_size;
    if diff == 0 {
        return bytes / frame_size;
    }


    let next_bound = frame_size - diff;
    (bytes + next_bound) / frame_size
}


pub trait Align<T> {
    fn align_up(&self, align: usize) -> Option<T>;
}



impl Align<usize> for usize {
    fn align_up(&self, align: usize) -> Option<usize> {
        if *self == 0 {
            return Some(align);
        }

        let align_mask = align - 1;
        if self & align_mask == 0 {
            Some(*self)
        } else {
            (self | align_mask).checked_add(1)
        }
    }
}

impl Align<u64> for u64 {
    fn align_up(&self, align: usize) -> Option<u64> {
        if *self == 0 {
            return u64::try_from(align).ok();
        }

        let align_mask = u64::try_from(align - 1).ok()?;
        if self & align_mask == 0 {
            Some(*self)
        } else {
            (self | align_mask).checked_add(1)
        }
    }
}


#[cfg(test)]
mod tests {
    use core::assert_eq;

    use crate::math::frame_count_from_bytes;

    const FRAME_SIZE: usize = 4096;

    #[test]
    fn it_frame_size_1() {
        assert_eq!(frame_count_from_bytes(1, FRAME_SIZE), 1);
    }


    #[test]
    fn it_frame_size_2() {
        assert_eq!(frame_count_from_bytes(FRAME_SIZE * 2, FRAME_SIZE), 2);
    }


    #[test]
    fn it_frame_size_3() {
        assert_eq!(frame_count_from_bytes((FRAME_SIZE * 2) + 1, FRAME_SIZE), 3);
    }
}
