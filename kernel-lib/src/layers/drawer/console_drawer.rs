use crate::error::KernelResult;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use core::any::Any;

use crate::gop::console::console_writer::ConsoleWriter;
use crate::gop::pixel::writer::enum_pixel_writer::EnumPixelWriter;
use crate::layers::drawer::console_drawer::console_colors::ConsoleColors;
use crate::layers::drawer::LayerDrawable;

pub mod console_colors;

pub struct ConsoleDrawer {
    console: ConsoleWriter,
}


impl ConsoleDrawer {
    pub fn new(frame_buffer_config: FrameBufferConfig, colors: ConsoleColors) -> Self {
        Self {
            console: ConsoleWriter::new(frame_buffer_config, *colors.foreground()),
        }
    }
}


impl LayerDrawable for ConsoleDrawer {
    fn draw_in_area(
        &mut self,
        pixels: &mut [u8],
        pixel_writer: &mut EnumPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        todo!()
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
