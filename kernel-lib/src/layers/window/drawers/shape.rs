use core::any::Any;

use common_lib::frame_buffer::PixelFormat;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::row::enum_pixel_converter::EnumPixelConverter;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;
use crate::layers::window::drawers::rect_colors::RectColors;
use crate::layers::window::drawers::WindowDrawable;

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


impl WindowDrawable for ShapeDrawer {
    fn draw_in_area(
        &mut self,
        _window_transform: &Transform2D,
        draw_rect: &Rectangle<usize>,
        writer: &mut dyn PixelFlushable,
    ) -> KernelResult {
        unsafe {
            writer.flush(PixelFrame::rect(
                *draw_rect,
                self.colors,
                self.converter.clone(),
            ))
        }
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }
}
