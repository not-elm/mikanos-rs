use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::vector::Vector2D;

use crate::error::KernelError::ExceededFrameBufferSize;
use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;

pub mod mapper;
pub mod pixel_color;
pub mod pixel_frame;
pub mod pixel_iter;
pub mod pixel_row;
pub mod writer;


#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    color: Option<PixelColor>,
    pos: Vector2D<usize>,
}


impl Pixel {
    #[inline(always)]
    pub const fn new(color: Option<PixelColor>, pos: Vector2D<usize>) -> Self {
        Self { color, pos }
    }


    #[inline(always)]
    pub fn color(&self) -> Option<PixelColor> {
        self.color
    }


    #[inline(always)]
    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
    }
}


impl Default for Pixel {
    fn default() -> Self {
        Self::new(Some(PixelColor::black()), Vector2D::zeros())
    }
}


impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.color() == other.color
    }
}


#[inline(always)]
pub(crate) fn calc_pixel_pos_from_vec2d(
    frame_buffer_config: &FrameBufferConfig,
    pos: &Vector2D<usize>,
) -> KernelResult<usize> {
    calc_pixel_pos(frame_buffer_config, pos.x(), pos.y())
}


pub(crate) fn calc_pixel_pos(
    frame_buffer_config: &FrameBufferConfig,
    x: usize,
    y: usize,
) -> KernelResult<usize> {
    if x > frame_buffer_config.horizontal_resolution || y > frame_buffer_config.vertical_resolution
    {
        return Err(ExceededFrameBufferSize);
    }

    Ok(4 * (frame_buffer_config.horizontal_resolution * y + x))
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
