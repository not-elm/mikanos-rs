use core::cmp::min;
use core::fmt::Error;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::abs::abs;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;

use self::console_colors::TextColors;
use self::console_frame::TextFrame;

pub mod console_colors;
mod console_frame;
mod console_row;


#[derive(Delegate)]
pub struct TextLayer {
    #[to(Transformable2D)]
    transform: Transform2D,
    config: FrameBufferConfig,
    text_frame: TextFrame<AscIICharWriter>,
}


impl TextLayer {
    pub fn new(
        config: FrameBufferConfig,
        pos: Vector2D<usize>,
        text_frame_size: Size,
        colors: TextColors,
    ) -> Self {
        let ascii = AscIICharWriter::new();
        let text_unit = ascii.font_unit();
        let transform = Transform2D::new(pos, text_unit * text_frame_size);

        Self {
            transform,
            text_frame: TextFrame::new(colors, ascii, text_frame_size, config.pixel_format),
            config,
        }
    }


    #[inline(always)]
    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.text_frame
            .update_string(str)
    }


    #[inline(always)]
    pub fn delete_last(&mut self) {
        self.text_frame
            .delete_last()
    }


    #[inline(always)]
    pub fn into_enum(self) -> Layer {
        Layer::Text(self)
    }
}


impl core::fmt::Write for TextLayer {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.text_frame
            .append_string(s)
            .map_err(|_| Error)
    }
}


impl LayerUpdatable for TextLayer {
    fn update_back_buffer(
        &mut self,
        back_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        let origin = self.transform.pos();
        let diff_y = abs(origin.y() as isize - draw_area.origin().y() as isize);
        let diff_x = abs(origin.x() as isize - draw_area.origin().x() as isize);

        for (y, line) in self
            .text_frame
            .frame_buff_lines()
            .into_iter()
            .flatten()
            .enumerate()
            .skip_while(|(y, _)| diff_y != *y)
            .take_while(|(y, _)| origin.y() + y <= draw_area.end().y())
        {
            let pos = self.pos() + Vector2D::new(diff_x, y);

            let origin = calc_pixel_pos(&self.config, pos.x(), pos.y())?;
            let len = min(line.len() - diff_x * 4, draw_area.size().width() * 4);

            let end = origin + len;

            back_buff.raw_mut()[origin..end].copy_from_slice(&line[diff_x * 4..(diff_x * 4 + len)]);
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
    use crate::layers::layer_updatable::LayerUpdatable;
    use crate::layers::text::console_colors::TextColors;
    use crate::layers::text::TextLayer;

    #[test]
    fn it_layer_size() {
        let console = TextLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::zeros(),
            Size::new(10, 10),
            TextColors::default(),
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
        let mut layer = TextLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::zeros(),
            Size::new(10, 10),
            TextColors::default(),
        );
        let mut back_buff = ShadowFrameBuffer::new(FrameBufferConfig::mock());

        layer
            .update_back_buffer(&mut back_buff, &layer.rect())
            .unwrap();
    }
}
