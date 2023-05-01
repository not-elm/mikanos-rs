use core::any::Any;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::gop::pixel::Pixel;
use crate::layers::drawer::rect_colors::RectColors;
use crate::layers::drawer::LayerDrawable;

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
        _window_transform: &Transform2D,
        pixels: &mut [PixelColor],
        draw_rect: &Rectangle<usize>,
    ) -> KernelResult {
        let rect_iter = draw_rect.points_unbound();
        let points = rect_iter.map(move |p| Pixel::new(Some(self.colors.foreground()), p));
        points
            .enumerate()
            .for_each(|(index, p)| {
                p.color().inspect(|color| {
                    pixels[index] = *color;
                });
            });

        Ok(())
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
