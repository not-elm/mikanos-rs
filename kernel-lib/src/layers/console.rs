use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::math::Align;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};
use console_colors::ConsoleColors;

use crate::apic::device_config::LocalApicTimerDivide;
use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;

pub mod console_colors;
mod console_frame;
mod console_row;
mod text_frame;
mod text_row;

#[derive(Delegate)]
pub struct ConsoleLayer {
    #[to(Transformable2D)]
    transform: Transform2D,
    config: FrameBufferConfig,
    ascii: FrameBufferPixelWriter,
    text_frame: TextFrame,
}


impl ConsoleLayer {
    pub fn new(
        config: FrameBufferConfig,
        pos: Vector2D<usize>,
        font_frame_size: Size,
        colors: ConsoleColors,
    ) -> Self {
        let ascii = AscIICharWriter::new();
        let font_unit = ascii.font_unit();

        let transform = Transform2D::new(pos, font_unit * font_frame_size);


        Self {
            transform,

            config,
            ascii: FrameBufferPixelWriter::new(config),
            text_frame: TextFrame::new(font_frame_size),
        }
    }


    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.text_frame
            .update_string(str);

        Ok(())
    }


    pub fn resize(&mut self, layer_size: Size) {
        // self.frame
        //     .resize_text_frame(calc_text_frame_size(layer_size,
        // self.font_unit))
    }


    pub fn into_enum(self) -> Layer {
        Layer::Console(self)
    }
}


impl core::fmt::Write for ConsoleLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.text_frame
            .append_string(s);


        Ok(())
    }
}

use crate::layers::console::text_frame::TextFrame;
use alloc::vec::Vec;

impl LayerUpdatable for ConsoleLayer {
    fn update_back_buffer(
        &mut self,
        back_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        let colors: Vec<&Vec<Option<PixelColor>>> = self
            .text_frame
            .rows_ref()
            .iter()
            .flat_map(|row| row.text_colors())
            .collect();

        let relative: Rectangle<usize> = draw_area.safe_sub_pos(&self.transform.pos());

        let origin = relative.origin();
        let end = relative.end();
        for y in origin.y()..end.y() {
            for x in origin.x()..end.x() {
                if let Some(color) = colors
                    .get(y)
                    .and_then(|line: &&Vec<Option<PixelColor>>| line.get(x))
                    .and_then(|color: &Option<PixelColor>| color.as_ref())
                {
                    let draw_pos = Vector2D::new(x, y) + self.transform.pos();
                    unsafe {
                        self.ascii
                            .write(back_buff.raw_mut(), &draw_pos, color)?
                    };
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use common_lib::frame_buffer::FrameBufferConfig;
    use common_lib::math::rectangle::Rectangle;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::transform2d::Transformable2D;

    use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
    use crate::layers::console::console_colors::ConsoleColors;
    use crate::layers::console::ConsoleLayer;
    use crate::layers::layer_updatable::LayerUpdatable;

    #[test]
    fn it_layer_size() {
        let console = ConsoleLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::zeros(),
            Size::new(10, 10),
            ConsoleColors::default(),
        );

        let size = console.rect().size();
        assert_eq!(size, Size::new(80, 160));
        assert_eq!(console.pos(), Vector2D::zeros());
        assert_eq!(
            console.rect(),
            Rectangle::new(Vector2D::zeros(), Vector2D::new(80, 160))
        )
    }


    #[test]
    fn it_update() {
        let mut layer = ConsoleLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::zeros(),
            Size::new(10, 10),
            ConsoleColors::default(),
        );
        let mut back_buff = ShadowFrameBuffer::new(FrameBufferConfig::mock());

        layer
            .update_back_buffer(&mut back_buff, &layer.rect())
            .unwrap();
    }
}
