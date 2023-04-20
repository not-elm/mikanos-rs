use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;

use crate::error::KernelError::ExceededFrameBufferSize;
use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub mod bgr_pixel_writer;
mod enum_pixel_writer;
pub(crate) mod mock_pixel_writer;
pub mod pixel_color;
pub mod pixel_writable;
pub mod rgb_pixel_writer;

#[cfg(feature = "alloc")]
pub mod mock_buffer_pixel_writer;

pub fn select_pixel_writer(frame_buffer_config: FrameBufferConfig) -> impl PixelWritable {
    #[cfg(not(test))]
    match frame_buffer_config.pixel_format {
        common_lib::frame_buffer::PixelFormat::Rgb => {
            crate::gop::pixel::enum_pixel_writer::EnumPixelWriter::Rgb(frame_buffer_config)
        }
        common_lib::frame_buffer::PixelFormat::Bgr => {
            crate::gop::pixel::enum_pixel_writer::EnumPixelWriter::Bgr(frame_buffer_config)
        }
    }

    #[cfg(test)]
    crate::gop::pixel::mock_pixel_writer::MockPixelWriter::new(frame_buffer_config)
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
) -> crate::error::KernelResult<usize> {
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
        assert!(calc_pixel_pos(&config, 0, 0).map(|p| p == 0).is_ok())
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
