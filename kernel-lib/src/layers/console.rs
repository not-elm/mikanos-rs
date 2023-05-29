use core::cmp::min;

use auto_delegate::Delegate;

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::frame_buffer_pixel_writer::FrameBufferPixelWriter;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;
use crate::gop::pixel::{calc_pixel_pos, calc_pixel_pos_from_vec2d};
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::serial_println;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

pub mod console_colors;
mod console_frame;
mod console_row;
mod text_frame;
mod text_row;

#[derive(Delegate)]
pub struct ConsoleLayer {
    #[to(Transformable2D)]
    transform: Transform2D,

    console_frame: ConsoleFrame<AscIICharWriter>,
    config: FrameBufferConfig,
}


impl ConsoleLayer {
    pub fn new(
        config: FrameBufferConfig,
        colors: ConsoleColors,
        pos: Vector2D<usize>,
        font_frame_size: Size,
    ) -> Self {
        let ascii = AscIICharWriter::new();
        let font_unit = ascii.font_unit();
        let transform = Transform2D::new(pos, font_unit * font_frame_size);


        Self {
            transform,
            console_frame: ConsoleFrame::new(
                ConsoleColors::default(),
                AscIICharWriter::new(),
                font_frame_size,
                config.pixel_format,
            ),
            config,
        }
    }


    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.console_frame
            .update_string(str);

        Ok(())
    }


    pub fn into_enum(self) -> Layer {
        Layer::Console(self)
    }
}


impl core::fmt::Write for ConsoleLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.console_frame
            .append_string(s);


        Ok(())
    }
}


use self::console_colors::ConsoleColors;
use self::console_frame::ConsoleFrame;


impl LayerUpdatable for ConsoleLayer {
    fn update_back_buffer(
        &mut self,
        back_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for (y, line) in self
            .console_frame
            .frame_buff_lines(0)
            .into_iter()
            .flatten()
            .enumerate()
            .skip_while(|(y, _)| *y < draw_area.origin().y())
        {
            let x = draw_area.origin().x() - self.transform.pos().x();
            if line.len() <= x {
                continue;
            }
            let pos = self.pos() + Vector2D::new(x, y);
            // let draw_origin = calc_pixel_pos_from_vec2d(&self.config,
            // &draw_area.origin())?; let draw_end =
            // calc_pixel_pos_from_vec2d(&self.config, &draw_area.end());

            let origin = calc_pixel_pos(&self.config, pos.x(), pos.y())?;
            let len = min(line.len() - x * 4, draw_area.size().width() * 4);

            let end = origin + len;

            back_buff.raw_mut()[origin..end].copy_from_slice(&line[x..(x + len)]);
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
