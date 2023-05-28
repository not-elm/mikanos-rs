use auto_delegate::Delegate;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use super::layer::Layer;
use super::layer_updatable::LayerUpdatable;
use crate::error::KernelResult;
use crate::gop::pixel::mapper::enum_pixel_mapper::EnumPixelMapper;
use crate::gop::pixel::mapper::PixelMapper;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
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
        _draw_area: &common_lib::math::rectangle::Rectangle<usize>,
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

        for (y, line) in CLOSE_BUTTON
            .iter()
            .enumerate()
        {
            for (x, c) in line.iter().enumerate() {
                let color = color_from_close_button_char(char::from(*c));
                let pos = Vector2D::new(x + origin.x(), y + origin.y());
                self.pixel_mapper
                    .write_frame_buff(&self.config, back_buff.raw_mut(), &pos, &color)?;
            }
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
