use crate::error::KernelResult;
use common_lib::frame_buffer::FrameBufferConfig;

use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub(crate) struct MockPixelWriter {}

impl MockPixelWriter {
    #[allow(dead_code)]
    pub fn new(_frame_buffer_config: FrameBufferConfig) -> Self {
        Self {}
    }
}

impl Drop for MockPixelWriter {
    fn drop(&mut self) {}
}

impl PixelWritable for MockPixelWriter {
    unsafe fn write(&mut self, _x: usize, _y: usize, _color: &PixelColor) -> KernelResult {
        Ok(())
    }
}
