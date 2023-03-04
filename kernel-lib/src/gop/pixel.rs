use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};

use crate::error::KernelError::ExceededFrameBufferSize;

use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::gop::pixel::rgb_pixel_writer::RgbPixelWriter;

pub mod gbr_pixel_writer;
pub mod pixel_color;
pub mod pixel_writable;
pub mod rgb_pixel_writer;

pub fn select_writer_from(frame_buffer_config: FrameBufferConfig) -> impl PixelWritable {
    match frame_buffer_config.pixel_format {
        PixelFormat::Rgb => RgbPixelWriter::new(frame_buffer_config),
        PixelFormat::Bgr => RgbPixelWriter::new(frame_buffer_config),
    }
}

fn calc_pixel_pos(
    frame_buffer_config: &FrameBufferConfig,
    x: usize,
    y: usize,
) -> crate::error::KernelResult<usize> {
    if x >= frame_buffer_config.horizontal_resolution
        || y >= frame_buffer_config.vertical_resolution
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
        assert!(calc_pixel_pos(&config, 0, 2).is_err())
    }
}
