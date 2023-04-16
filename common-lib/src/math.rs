pub fn frame_size_from_bytes(bytes: usize, frame_size: usize) -> usize {
    if frame_size == 0 {
        return bytes;
    }

    let diff = bytes % frame_size;
    if diff == 0 {
        return bytes / frame_size;
    }


    let next_bound = frame_size - diff;
    (bytes + next_bound) / frame_size
}


#[cfg(test)]
mod tests {
    use crate::math::frame_size_from_bytes;

    const FRAME_SIZE: usize = 4096;

    #[test]
    fn it_frame_size_1() {
        assert_eq!(frame_size_from_bytes(1, FRAME_SIZE), 1);
    }


    #[test]
    fn it_frame_size_2() {
        assert_eq!(frame_size_from_bytes(FRAME_SIZE * 2, FRAME_SIZE), 2);
    }


    #[test]
    fn it_frame_size_3() {
        assert_eq!(frame_size_from_bytes((FRAME_SIZE * 2) + 1, FRAME_SIZE), 3);
    }
}
