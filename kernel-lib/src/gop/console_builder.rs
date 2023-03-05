use crate::gop::console::Console;
use crate::gop::pixel::pixel_color::PixelColor;
use common_lib::frame_buffer::FrameBufferConfig;

pub struct ConsoleBuilder {
    color: Option<PixelColor>,
}

impl ConsoleBuilder {
    pub fn new() -> Self {
        Self { color: None }
    }

    pub fn color(mut self, color: PixelColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn build(self, frame_buffer_config: FrameBufferConfig) -> Console {
        Console::new(
            frame_buffer_config,
            self.color.unwrap_or(PixelColor::new(0xFF, 0xFF, 0xFF)),
        )
    }
}
