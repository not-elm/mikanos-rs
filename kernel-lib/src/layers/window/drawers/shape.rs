use core::any::Any;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::drawers::WindowDrawable;
use common_lib::transform::Transform2D;

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
    fn draw(&mut self, transform: &Transform2D, writer: &mut dyn PixelWritable) -> KernelResult {
        fill_rect(writer, transform, &self.color)
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
    transform: &Transform2D,
    color: &PixelColor,
) -> KernelResult {
    let origin = transform.pos();
    let dest = transform.pos() + transform.size();

    for y in origin.y()..=dest.y() {
        for x in origin.x()..=dest.x() {
            unsafe { pixel_writer.write(x, y, color) }?;
        }
    }
    Ok(())
}
