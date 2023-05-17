use core::fmt::Error;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use console_colors::ConsoleColors;

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::char::char_writable::CharWritable;
use crate::gop::pixel::calc_pixel_pos_from_vec2d;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::console::console_frame::ConsoleFrame;
use crate::layers::frame_buffer_layer_transform;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;

pub mod console_colors;
mod console_frame;
mod console_row;

#[derive(Delegate)]
pub struct ConsoleLayer {
    #[to(Transformable2D)]
    transform: Transform2D,
    frame: ConsoleFrame<AscIICharWriter>,
    font_unit: Size,
    config: FrameBufferConfig,
}


impl ConsoleLayer {
    pub fn new(config: FrameBufferConfig, colors: ConsoleColors) -> Self {
        let transform = frame_buffer_layer_transform(config);
        let ascii = AscIICharWriter::new();
        let font_unit = ascii.font_unit();
        let font_frame_size = calc_text_frame_size(transform.size(), font_unit);
        let frame = ConsoleFrame::new(colors, ascii, font_frame_size, config.pixel_format);

        Self {
            transform,
            frame,
            font_unit,
            config,
        }
    }


    pub fn resize(&mut self, layer_size: Size) {
        self.frame
            .resize_text_frame(calc_text_frame_size(layer_size, self.font_unit))
    }

    pub fn into_enum(self) -> Layer {
        Layer::Console(self)
    }
}


impl core::fmt::Write for ConsoleLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.frame
            .write_string(s)
            .map_err(|_| Error::default())
    }
}


impl LayerUpdatable for ConsoleLayer {
    fn update_shadow_buffer(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for (y, line) in self
            .frame
            .frame_buff_lines()
            .into_iter()
            .flatten()
            .enumerate()
        {
            let origin =
                calc_pixel_pos_from_vec2d(&self.config, &Vector2D::new(draw_area.origin().x(), y))?;

            let end = origin + line.len();

            shadow_buff.raw_mut()[origin..end].copy_from_slice(line);
        }
        Ok(())
    }
}


fn calc_text_frame_size(layer_size: Size, font_unit_size: Size) -> Size {
    layer_size / font_unit_size
}


#[cfg(test)]
mod tests {
    use common_lib::math::size::Size;

    use crate::layers::console::calc_text_frame_size;

    #[test]
    fn it_font_frame_size() {
        let font_frame_size = calc_text_frame_size(Size::new(80, 160), Size::new(8, 16));
        assert_eq!(font_frame_size, Size::new(10, 10));

        let font_frame_size = calc_text_frame_size(Size::new(83, 163), Size::new(8, 16));
        assert_eq!(font_frame_size, Size::new(10, 10));
    }
}
