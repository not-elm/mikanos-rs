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
use crate::layers::text::config::TextConfig;

use self::colors::TextColors;
use self::frame::TextFrame;

pub mod colors;
pub mod command;
mod command_history;
pub mod config;
pub mod frame;
mod row;


#[derive(Delegate)]
pub struct TextLayer {
    #[to(Transformable2D)]
    transform: Transform2D,
    frame_buffer_config: FrameBufferConfig,
    text_frame: TextFrame,
}


impl TextLayer {
    pub fn new(
        frame_buffer_config: FrameBufferConfig,
        pos: Vector2D<usize>,
        text_frame_size: Size,
        config: TextConfig,
    ) -> KernelResult<Self> {
        let ascii = AscIICharWriter::new();
        let text_unit = ascii.font_unit();
        let transform = Transform2D::new(pos, text_unit * text_frame_size);

        Ok(Self {
            transform,
            text_frame: TextFrame::new(
                ascii,
                text_frame_size,
                frame_buffer_config.pixel_format,
                config,
            )?,
            frame_buffer_config,
        })
    }


    #[inline(always)]
    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.text_frame
            .update_string(str)
    }


    #[inline(always)]
    pub fn delete_last(&mut self) {
        self.text_frame.delete_last()
    }


    #[inline(always)]
    pub fn change_colors(&mut self, colors: TextColors) -> KernelResult {
        self.text_frame
            .change_colors(colors)
    }


    #[inline(always)]
    pub fn text_cursor_pos(&self) -> Vector2D<usize> {
        self.text_frame
            .text_cursor_pos()
    }

    #[inline]
    pub fn history_up(&mut self) -> KernelResult {
        self.text_frame.history_up()
    }


    #[inline]
    pub fn history_down(&mut self) -> KernelResult {
        self.text_frame.history_down()
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
        let src_buff = self
            .text_frame
            .frame_buff_lines()
            .into_iter()
            .flatten();

        update(
            &self.frame_buffer_config,
            back_buff,
            draw_area,
            self.pos(),
            src_buff,
        )
    }
}


pub(crate) fn update<'a>(
    config: &FrameBufferConfig,
    back_buff: &mut ShadowFrameBuffer,
    draw_area: &Rectangle<usize>,
    origin: Vector2D<usize>,
    src_buff: impl Iterator<Item = &'a [u8]>,
) -> KernelResult {
    let diff_y = abs(origin.y() as isize - draw_area.origin().y() as isize);
    let diff_x = abs(origin.x() as isize - draw_area.origin().x() as isize);

    for (y, line) in src_buff
        .enumerate()
        .skip_while(|(y, _)| diff_y != *y)
        .take_while(|(y, _)| origin.y() + y <= draw_area.end().y())
    {
        let pos = origin + Vector2D::new(diff_x, y);

        let origin = calc_pixel_pos(config, pos.x(), pos.y())?;
        let len = min(line.len() - diff_x * 4, draw_area.size().width() * 4);
        let end = origin + len;

        back_buff.raw_mut()[origin..end].copy_from_slice(&line[diff_x * 4..(diff_x * 4 + len)]);
    }

    Ok(())
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
    use crate::layers::text::{config, TextLayer};

    #[test]
    fn it_layer_size() {
        let config = config::Builder::new()
            .set_scrollable()
            .build();

        let console = TextLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::zeros(),
            Size::new(10, 10),
            config,
        )
        .unwrap();

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
        let config = config::Builder::new()
            .set_scrollable()
            .build();
        let mut layer = TextLayer::new(
            FrameBufferConfig::mock(),
            Vector2D::zeros(),
            Size::new(10, 10),
            config,
        )
        .unwrap();
        let mut back_buff = ShadowFrameBuffer::new(FrameBufferConfig::mock());

        layer
            .update_back_buffer(&mut back_buff, &layer.rect())
            .unwrap();
    }
}
