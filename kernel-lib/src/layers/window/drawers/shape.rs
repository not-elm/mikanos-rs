use core::any::Any;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
use crate::layers::window::drawers::WindowDrawable;

#[derive(Debug, Clone)]
pub struct ShapeWDrawer {
    color: PixelColor,
}


impl ShapeWDrawer {
    pub fn new(color: PixelColor) -> Self {
        Self { color }
    }
}


impl WindowDrawable for ShapeWDrawer {
    fn draw_in_area(
        &mut self,
        _window_transform: &Transform2D,
        draw_rect: &Rectangle<usize>,
        writer: &mut dyn PixelFlushable,
    ) -> KernelResult {
        unsafe { writer.flush(PixelFrame::rect(*draw_rect, self.color)) }
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }
}


impl Default for ShapeWDrawer {
    fn default() -> Self {
        Self::new(PixelColor::black())
    }
}


fn fill_rect(
    pixel_writer: &mut dyn PixelWritable,
    rect: &Rectangle<usize>,
    color: &PixelColor,
) -> KernelResult {
    let origin = rect.origin();
    let end = rect.end();

    for y in origin.y()..=end.y() {
        for x in origin.x()..=end.x() {
            unsafe { pixel_writer.write(x, y, color) }?;
        }
    }
    Ok(())
}
