use alloc::vec;
use alloc::vec::Vec;

use common_lib::frame_buffer::FrameBufferConfig;

#[derive(Debug)]
pub struct ShadowFrameBuffer {
    buff: Vec<u8>,
    config: FrameBufferConfig,
}


impl ShadowFrameBuffer {
    pub fn new(config: FrameBufferConfig) -> Self {
        Self {
            buff: vec![0; config.frame_buffer_size],
            config,
        }
    }


    pub const fn width(&self) -> usize {
        self.config
            .horizontal_resolution
    }


    pub const fn height(&self) -> usize {
        self.config
            .vertical_resolution
    }


    pub fn raw_ref(&self) -> &[u8] {
        self.buff.as_slice()
    }


    pub fn config_ref(&self) -> &FrameBufferConfig {
        &self.config
    }


    pub fn raw_mut(&mut self) -> &mut [u8] {
        self.buff.as_mut_slice()
    }
}
