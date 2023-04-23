use alloc::rc::Rc;
use core::cell::RefCell;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;
use writer::pixel_writable::PixelWritable;
use writer::rgb_pixel_writer::RgbPixelWriter;

use crate::error::KernelError::ExceededFrameBufferSize;
use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;

pub mod pixel_color;


pub mod writer;


pub mod pixel_frame;
pub mod pixel_iter;


#[derive(Debug, Clone)]
pub struct Pixel {
    color: Option<PixelColor>,
    pos: Vector2D<usize>,
}


impl Pixel {
    pub const fn new(color: Option<PixelColor>, pos: Vector2D<usize>) -> Self {
        Self { color, pos }
    }

    pub fn color(&self) -> Option<PixelColor> {
        self.color
    }


    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
    }
}


pub fn rc_pixel_writer(frame_buffer_config: FrameBufferConfig) -> Rc<RefCell<dyn PixelWritable>> {
    match frame_buffer_config.pixel_format {
        common_lib::frame_buffer::PixelFormat::Rgb => {
            Rc::new(RefCell::new(RgbPixelWriter::new(frame_buffer_config)))
        }
        common_lib::frame_buffer::PixelFormat::Bgr => {
            Rc::new(RefCell::new(RgbPixelWriter::new(frame_buffer_config)))
        }
    }
}

pub fn select_pixel_writer(frame_buffer_config: FrameBufferConfig) -> impl PixelWritable {
    #[cfg(not(test))]
    use crate::gop::pixel::writer::enum_pixel_writer;
    #[cfg(test)]
    use crate::gop::pixel::writer::mock_pixel_writer;

    #[cfg(not(test))]
    match frame_buffer_config.pixel_format {
        common_lib::frame_buffer::PixelFormat::Rgb => {
            enum_pixel_writer::EnumPixelWriter::Rgb(frame_buffer_config)
        }
        common_lib::frame_buffer::PixelFormat::Bgr => {
            enum_pixel_writer::EnumPixelWriter::Bgr(frame_buffer_config)
        }
    }

    #[cfg(test)]
    mock_pixel_writer::MockPixelWriter::new(frame_buffer_config)
}

pub fn fill_rect(
    pixel_writer: &mut impl PixelWritable,
    origin: Vector2D<usize>,
    to: Vector2D<usize>,
    color: PixelColor,
) -> KernelResult {
    for y in origin.y()..=to.y() {
        for x in origin.x()..=to.x() {
            unsafe { pixel_writer.write(x, y, &color) }?;
        }
    }
    Ok(())
}

fn calc_pixel_pos(
    frame_buffer_config: &FrameBufferConfig,
    x: usize,
    y: usize,
) -> KernelResult<usize> {
    if x > frame_buffer_config.horizontal_resolution || y > frame_buffer_config.vertical_resolution
    {
        return Err(ExceededFrameBufferSize);
    }

    Ok(4 * (frame_buffer_config.pixel_per_scanline * y + x))
}

#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};

    use crate::gop::pixel::calc_pixel_pos;

    #[test]
    fn it_works() {
        let config = FrameBufferConfig::new(0, 3, 6, 3, 2, PixelFormat::Rgb);
        assert!(calc_pixel_pos(&config, 0, 0)
            .map(|p| p == 0)
            .is_ok())
    }

    #[test]
    fn it_over_x() {
        let config = FrameBufferConfig::new(0, 3, 6, 3, 2, PixelFormat::Rgb);
        assert!(calc_pixel_pos(&config, 5, 0).is_err())
    }

    #[test]
    fn it_over_y() {
        let config = FrameBufferConfig::new(0, 3, 6, 3, 2, PixelFormat::Rgb);
        assert!(calc_pixel_pos(&config, 0, 3).is_err())
    }
}
