use alloc::vec;
use alloc::vec::Vec;

use common_lib::frame_buffer::FrameBufferConfig;

use crate::error::{KernelError, KernelResult};
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub(crate) struct MockBufferPixelWriter {
    buff: Vec<Vec<u8>>,
}

impl MockBufferPixelWriter {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buff: vec![vec![0; width]; height]
        }
    }

    pub fn height(&self) -> usize {
        self.buff.len()
    }

    pub fn width(&self) -> usize {
        self.buff[0].len()
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> u8 {
        self.buff[y][x]
    }
}

impl Drop for MockBufferPixelWriter {
    fn drop(&mut self) {}
}

impl PixelWritable for MockBufferPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        if self.height() <= y || self.width() + 2 <= x {
            return Err(KernelError::ExceededFrameBufferSize);
        }
        self.buff[y][x] = color.r();
        self.buff[y][x + 1] = color.g();
        self.buff[y][x + 2] = color.b();

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::gop::pixel::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::pixel_writable::PixelWritable;

    #[test]
    fn it_new() {
        let mock = MockBufferPixelWriter::new(3, 5);
        assert_eq!(mock.buff.len(), 5);
        mock.buff.iter().for_each(|line| {
            assert_eq!(line.len(), 3)
        });
    }


    #[test]
    fn it_write() {
        let mut mock = MockBufferPixelWriter::new(10, 10);
        unsafe { assert!(mock.write(0, 0, &PixelColor::new(0xFF, 0x33, 0x11)).is_ok()); }
        assert_eq!(mock.buff[0][0], 0xFF);
        assert_eq!(mock.buff[0][1], 0x33);
        assert_eq!(mock.buff[0][2], 0x11);
    }
}