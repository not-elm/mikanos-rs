use core::any::Any;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::console::console_builder::ConsoleBuilder;
use crate::gop::console::console_writer::ConsoleWriter;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::layers::drawer::console_drawer::console_colors::ConsoleColors;
use crate::layers::drawer::LayerDrawable;

pub mod console_colors;

pub struct ConsoleDrawer {
    console: ConsoleWriter<AscIICharWriter>,
}


impl ConsoleDrawer {
    pub fn new(frame_buffer_config: FrameBufferConfig, colors: ConsoleColors) -> Self {
        Self {
            console: ConsoleBuilder::new()
                .color(*colors.foreground())
                .build(frame_buffer_config),
        }
    }
}


impl LayerDrawable for ConsoleDrawer {
    fn draw_in_area(
        &mut self,
        dist_buff: &mut [u8],
        pixel_writer: &mut FrameBufferPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        // self.console.write_str()
        todo!()
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
