use common_lib::frame_buffer::FrameBufferConfig;

use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::console::console_writer::ConsoleWriter;
use crate::gop::pixel::pixel_color::PixelColor;

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

    pub fn build(self, frame_buffer_config: FrameBufferConfig) -> ConsoleWriter<AscIICharWriter> {
        ConsoleWriter::new(
            frame_buffer_config,
            AscIICharWriter::default(),
            self.color
                .unwrap_or(PixelColor::new(0xFF, 0xFF, 0xFF)),
        )
    }
}

impl Default for ConsoleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
