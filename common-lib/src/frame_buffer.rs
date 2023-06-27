use crate::math::rectangle::Rectangle;
use crate::math::size::Size;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FrameBufferConfig {
    pub frame_buffer_base_addr: u64,
    pub frame_buffer_size: usize,
    pub pixel_per_scanline: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
    pub pixel_format: PixelFormat,
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}


impl FrameBufferConfig {
    #[inline(always)]
    pub fn new(
        frame_buffer_base: u64,
        frame_buffer_size: usize,
        pixel_per_scanline: usize,
        horizontal_resolution: usize,
        vertical_resolution: usize,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            frame_buffer_base_addr: frame_buffer_base,
            frame_buffer_size,
            pixel_per_scanline,
            horizontal_resolution,
            vertical_resolution,
            pixel_format,
        }
    }


    #[inline(always)]
    pub fn frame_size(&self) -> Size {
        Size::new(self.horizontal_resolution, self.vertical_resolution)
    }


    #[inline(always)]
    pub fn frame_rect(&self) -> Rectangle<usize> {
        Rectangle::from_size(self.frame_size())
    }


    #[inline(always)]
    pub fn frame_buff_pixels(&self) -> usize {
        self.horizontal_resolution * self.vertical_resolution
    }


    #[inline(always)]
    pub fn frame_buff_length(&self) -> usize {
        4 * self.frame_buff_pixels()
    }


    #[inline(always)]
    pub fn mock() -> Self {
        Self {
            frame_buffer_base_addr: 0,
            frame_buffer_size: 3145728,
            pixel_per_scanline: 1024,
            horizontal_resolution: 1024,
            vertical_resolution: 768,
            pixel_format: PixelFormat::Rgb,
        }
    }


    #[inline(always)]
    pub fn frame_buffer_base_ptr(&self) -> *mut u8 {
        self.frame_buffer_base_addr as *mut u8
    }
}
