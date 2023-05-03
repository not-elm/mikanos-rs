use alloc::vec;
use alloc::vec::Vec;

use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};

use crate::error::{KernelError, KernelResult};
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::writer::pixel_writable::{flush_frame_buff, PixelFlushable, PixelWritable};

#[allow(dead_code)]
pub(crate) struct MockBufferPixelWriter {
    width: usize,
    height: usize,
    buff: Vec<u8>,
}

impl MockBufferPixelWriter {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buff: vec![0; width * height],
        }
    }

    pub fn len(&self) -> usize {
        self.buff.len()
    }

    #[allow(dead_code)]
    pub fn pixel_at(&self, x: usize, y: usize) -> PixelColor {
        let pixel_pos = calc_pos(x, y, self.width);
        PixelColor::new(
            self.buff[pixel_pos],
            self.buff[pixel_pos + 1],
            self.buff[pixel_pos + 2],
        )
    }
}


fn calc_pos(x: usize, y: usize, width: usize) -> usize {
    4 * (width * y + x)
}


impl Drop for MockBufferPixelWriter {
    fn drop(&mut self) {}
}


impl PixelWritable for MockBufferPixelWriter {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        let pixel_pos = calc_pos(x, y, self.width);
        if self.len() <= pixel_pos + 2 {
            return Err(KernelError::ExceededFrameBufferSize);
        }
        self.buff[pixel_pos] = color.r();
        self.buff[pixel_pos + 1] = color.g();
        self.buff[pixel_pos + 2] = color.b();

        Ok(())
    }

    unsafe fn write_shadow_buff(
        &mut self,
        _buff: &mut [u8],
        _x: usize,
        _y: usize,
        _color: &PixelColor,
    ) -> KernelResult {
        todo!()
    }
}

impl PixelFlushable for MockBufferPixelWriter {
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
        flush_frame_buff(
            pixel_frame,
            &FrameBufferConfig::new(
                self.buff.as_ptr() as u64,
                self.len(),
                self.width,
                self.width,
                self.height,
                PixelFormat::Rgb,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::writer::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::gop::pixel::writer::pixel_writable::PixelWritable;

    #[test]
    fn it_new() {
        let mock = MockBufferPixelWriter::new(3, 5);
        assert_eq!(mock.buff.len(), 15);
    }


    #[test]
    fn it_write() {
        let mut mock = MockBufferPixelWriter::new(10, 10);
        let color = PixelColor::new(0xFF, 0x33, 0x11);
        unsafe {
            assert!(mock
                .write(0, 0, &color)
                .is_ok());
        }


        assert_eq!(mock.pixel_at(0, 0), color);
    }
}
