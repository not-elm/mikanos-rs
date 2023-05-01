use alloc::vec::Vec;
use core::any::Any;

use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::{calc_pixel_pos, calc_pixel_pos_from_vec2d, calc_shadow_buffer_pixel_pos_from_vec2d};
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::layers::drawer::LayerDrawable;
use crate::layers::drawer::rect_colors::RectColors;

#[derive(Debug, Clone)]
pub struct ShapeDrawer {
    colors: RectColors,
    converter: EnumPixelConverter,
}


impl ShapeDrawer {
    pub fn new(colors: RectColors, pixel_format: PixelFormat) -> Self {
        Self {
            colors,
            converter: EnumPixelConverter::new(pixel_format),
        }
    }
}


impl LayerDrawable for ShapeDrawer {
    fn draw_in_area(
        &mut self,
        config: &FrameBufferConfig,
        _window_transform: &Transform2D,
        pixels: &mut [PixelColor],
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        for p in draw_area.points() {
            pixels[calc_shadow_buffer_pixel_pos_from_vec2d(config, p)?] = self.colors.foreground();
        }

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
