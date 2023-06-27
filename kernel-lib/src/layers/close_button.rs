use alloc::vec::Vec;
use core::cmp::min;

use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::abs::abs;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;

use super::layer::Layer;
use super::layer_updatable::LayerUpdatable;

pub(crate) const CLOSE_BUTTON_WIDTH: usize = 16;
pub(crate) const CLOSE_BUTTON_HEIGHT: usize = 14;


#[derive(Delegate)]
pub struct CloseButtonLayer {
    #[to(Transformable2D)]
    transform: Transform2D,

    pixel_mapper: EnumPixelMapper,

    config: FrameBufferConfig,
}


impl CloseButtonLayer {
    pub fn new(config: FrameBufferConfig, pos: Vector2D<usize>) -> Self {
        Self {
            transform: Transform2D::new(pos, Size::new(CLOSE_BUTTON_WIDTH, CLOSE_BUTTON_HEIGHT)),
            pixel_mapper: EnumPixelMapper::new(config.pixel_format),
            config,
        }
    }


    pub fn into_enum(self) -> Layer {
        Layer::CloseButton(self)
    }
}


impl LayerUpdatable for CloseButtonLayer {
    fn update_back_buffer(
        &mut self,
        back_buff: &mut ShadowFrameBuffer,
        draw_area: &common_lib::math::rectangle::Rectangle<usize>,
    ) -> KernelResult {
        const CLOSE_BUTTON: [&[u8; CLOSE_BUTTON_WIDTH]; CLOSE_BUTTON_HEIGHT] = [
            b"...............@",
            b".:::::::::::::$@",
            b".:::::::::::::$@",
            b".:::@@::::@@::$@",
            b".::::@@::@@:::$@",
            b".:::::@@@@::::$@",
            b".::::::@@:::::$@",
            b".:::::@@@@::::$@",
            b".::::@@::@@:::$@",
            b".:::@@::::@@::$@",
            b".:::::::::::::$@",
            b".:::::::::::::$@",
            b".$$$$$$$$$$$$$$@",
            b"@@@@@@@@@@@@@@@@",
        ];

        let origin = self.transform.pos();
        let diff_y = abs(origin.y() as isize - draw_area.origin().y() as isize);
        let diff_x = abs(origin.x() as isize - draw_area.origin().x() as isize);

        for (y, line) in CLOSE_BUTTON
            .iter()
            .enumerate()
            .skip_while(|(y, _)| diff_y != *y)
            .take_while(|(y, _)| origin.y() + y <= draw_area.end().y())
        {
            let buff: Vec<u8> = line
                .iter()
                .map(|c| color_from_close_button_char(char::from(*c)))
                .flat_map(|color| {
                    self
                        .pixel_mapper
                        .convert_to_buff(&color)
                })
                .collect();

            let pos = self.pos() + Vector2D::new(diff_x, y);

            let origin = calc_pixel_pos(&self.config, pos.x(), pos.y())?;
            let len = min(buff.len() - diff_x * 4, draw_area.size().width() * 4);

            let end = origin + len;

            back_buff.raw_mut()[origin..end].copy_from_slice(&buff[diff_x * 4..(diff_x * 4 + len)]);
        }

        Ok(())
    }
}


fn color_from_close_button_char(c: char) -> PixelColor {
    match c {
        '@' => PixelColor::black(),
        '$' => PixelColor::new(0x84, 0x84, 0x84),
        ':' => PixelColor::new(0xC6, 0xC6, 0xC6),
        _ => PixelColor::white(),
    }
}


#[cfg(test)]
mod tests {
    use common_lib::{
        frame_buffer::FrameBufferConfig, math::vector::Vector2D,
        transform::transform2d::Transformable2D,
    };

    use crate::{
        gop::{pixel::pixel_color::PixelColor, shadow_frame_buffer::ShadowFrameBuffer},
        layers::{close_button::color_from_close_button_char, layer_updatable::LayerUpdatable},
    };

    use super::CloseButtonLayer;

    #[test]
    fn it_close_button_color() {
        assert_eq!(color_from_close_button_char('@'), PixelColor::black());
        assert_eq!(
            color_from_close_button_char('$'),
            PixelColor::new(0x84, 0x84, 0x84)
        );
        assert_eq!(
            color_from_close_button_char(':'),
            PixelColor::new(0xC6, 0xC6, 0xC6)
        );
        assert_eq!(color_from_close_button_char('.'), PixelColor::white());
    }


    #[test]
    fn it_update_back_buffer_when_draw_area_same_layer_rect() {
        let mut close_button = CloseButtonLayer::new(FrameBufferConfig::mock(), Vector2D::zeros());
        let mut back_buff = ShadowFrameBuffer::new(FrameBufferConfig::mock());
        close_button
            .update_back_buffer(&mut back_buff, &close_button.rect())
            .unwrap();
    }
}
