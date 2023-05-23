use core::fmt::Error;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::Align;
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
    pub fn new(
        config: FrameBufferConfig,
        transform: Transform2D,
        colors: ConsoleColors) -> Self {
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


    pub fn update_string(&mut self, str: &str) -> KernelResult {
        self.frame.update_string(str)
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
            .append_string(s)
            .map_err(|_| Error)
    }
}


impl LayerUpdatable for ConsoleLayer {
    fn update_shadow_buffer(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        let x = draw_area.origin().x();
        if let Some((io, ie)) = calc_text_line_range(&self.transform.rect(), draw_area, &self.font_unit) {
            for (iy, line) in self
                .frame
                .frame_buff_lines()
                .into_iter()
                .flatten()
                .enumerate()
            {
                let y = iy + draw_area.origin().y();
                if draw_area.end().y() < y {
                    return Ok(());
                }

                let origin = calc_pixel_pos_from_vec2d(&self.config, &Vector2D::new(x, y))?;
                let width = ie - io;

                shadow_buff.raw_mut()[origin..(origin + width)].copy_from_slice(&line[io..ie]);
            }
        }


        Ok(())
    }
}


fn calc_text_frame_size(layer_size: Size, font_unit_size: Size) -> Size {
    layer_size / font_unit_size
}


fn calc_text_line_range(
    layer_rec: &Rectangle<usize>,
    draw_rec: &Rectangle<usize>,
    font_unit: &Size,
) -> Option<(usize, usize)> {
    let lo = layer_rec.origin().x();
    let lo = if lo == 0 { 0 } else { lo.align_up(font_unit.width())? };

    let xo = if draw_rec.origin().x() == 0 { 0 } else { draw_rec.origin().x().align_up(font_unit.width())? };
    let io = xo.checked_sub(lo)? * 4;

    let text_len = draw_rec.end().x().checked_sub(xo)? / font_unit.width();

    let ie = io + text_len * font_unit.width() * 4;

    Some((io, ie))
}


#[cfg(test)]
mod tests {
    use common_lib::math::rectangle::Rectangle;
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;

    use crate::layers::console::{calc_text_frame_size, calc_text_line_range};

    #[test]
    fn it_font_frame_size() {
        let font_frame_size = calc_text_frame_size(Size::new(80, 160), Size::new(8, 16));
        assert_eq!(font_frame_size, Size::new(10, 10));

        let font_frame_size = calc_text_frame_size(Size::new(83, 163), Size::new(8, 16));
        assert_eq!(font_frame_size, Size::new(10, 10));
    }


    #[test]
    fn it_calc_text_line_range_one_text() {
        let layer_rect = Rectangle::from_size(Size::new(100, 100));
        let draw_area = Rectangle::new(Vector2D::new(8, 0), Vector2D::new(16, 0));
        let (io, ie) = calc_text_line_range(&layer_rect, &draw_area, &Size::new(8, 16)).unwrap();
        assert_eq!(io, 32);
        assert_eq!(ie, 64)
    }


    #[test]
    fn it_calc_text_line_range() {
        let layer_rect = Rectangle::from_size(Size::new(100, 100));
        let draw_area = Rectangle::new(Vector2D::new(3, 0), Vector2D::new(90, 0));
        let (io, ie) = calc_text_line_range(&layer_rect, &draw_area, &Size::new(8, 16)).unwrap();
        assert_eq!(io, 32);
        assert_eq!(ie, 352)
    }
}
