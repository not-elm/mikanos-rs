use alloc::vec;
use alloc::vec::Vec;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::shape_drawer::ShapeDrawer;

#[derive(Debug)]
pub struct ShadowFrameBuffer {
    buff: Vec<u8>,
    config: FrameBufferConfig,
}


impl ShadowFrameBuffer {
    #[inline(always)]
    pub fn new(config: FrameBufferConfig) -> Self {
        Self {
            buff: vec![0; config.frame_buff_length()],
            config,
        }
    }


    #[inline(always)]
    pub const fn width(&self) -> usize {
        self.config
            .horizontal_resolution
    }


    #[inline(always)]
    pub const fn height(&self) -> usize {
        self.config
            .vertical_resolution
    }


    pub fn fill_rect(
        &mut self,
        draw_area: &Rectangle<usize>,
        color: PixelColor,
    ) -> KernelResult {
        ShapeDrawer::new(self.config, color)
            .update_back_buffer(self, draw_area)
    }


    #[inline(always)]
    pub fn raw_ref(&self) -> &[u8] {
        self.buff.as_slice()
    }


    #[inline(always)]
    pub fn config_ref(&self) -> &FrameBufferConfig {
        &self.config
    }


    #[inline(always)]
    pub fn raw_mut(&mut self) -> &mut [u8] {
        self.buff.as_mut_slice()
    }
}
