use common_lib::frame_buffer::PixelFormat;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::{KernelError, KernelResult};
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;

#[derive(Debug)]
pub struct BuffPixelWriter {
    buff_size: Size,
    mapper: EnumPixelMapper,
}


impl BuffPixelWriter {
    pub const fn new(buff_size: Size, pixel_format: PixelFormat) -> Self {
        Self {
            buff_size,
            mapper: EnumPixelMapper::new(pixel_format),
        }
    }
}


impl PixelWritable for BuffPixelWriter {
    unsafe fn write(
        &mut self,
        buff: &mut [u8],
        pos: &Vector2D<usize>,
        color: &PixelColor,
    ) -> KernelResult {
        let origin = self.mapper.pixel_len() * pos.x() + (pos.y() * self.buff_size.width());
        let end = self.mapper.pixel_len() + origin;

        if buff.len() <= end {
            return Err(KernelError::ExceededFrameBufferSize);
        }

        buff[origin..end].copy_from_slice(
            self.mapper
                .convert_to_buff(color),
        );
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec;

    use common_lib::array::eq_array;
    use common_lib::frame_buffer::PixelFormat;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::pixel_color::PixelColor;
    use crate::gop::pixel::writer::buff_pixel_writer::BuffPixelWriter;
    use crate::gop::pixel::writer::pixel_writable::PixelWritable;

    #[test]
    fn it_write_buff() {
        let mut writer = BuffPixelWriter::new(Size::new(100, 100), PixelFormat::Bgr);
        let mut buff = vec![0; 100 * 100];
        unsafe {
            writer
                .write(
                    buff.as_mut_slice(),
                    &Vector2D::zeros(),
                    &PixelColor::white(),
                )
                .unwrap();
        }

        let mut expect = vec![0; 100 * 100];
        expect[0..4].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0x00]);

        assert!(eq_array(buff.as_slice(), expect.as_slice()))
    }


    #[test]
    fn it_write_from_2line() {
        let mut writer = BuffPixelWriter::new(Size::new(100, 100) * 4, PixelFormat::Rgb);

        let mut buff = vec![0; 400 * 400];
        unsafe {
            writer
                .write(
                    buff.as_mut_slice(),
                    &Vector2D::new(0, 2),
                    &PixelColor::white(),
                )
                .unwrap();
        }


        assert!(eq_array(&buff[800..804], &[0xFF, 0xFF, 0xFF, 0x00]));
    }
}
