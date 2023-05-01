use alloc::string::String;
use core::fmt::Debug;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::enum_pixel_writer::EnumPixelWriter;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
use crate::layers::drawer::LayerDrawable;

pub struct Layer<Writer, Draw> {
    key: String,
    layer_transform: Transform2D,
    pixel_writer: Writer,
    drawer: Draw,
}


impl<Writer, Draw> Layer<Writer, Draw> {
    pub fn new(key: &str, transform: Transform2D, pixel_writer: Writer, drawer: Draw) -> Self {
        Self {
            key: String::from(key),
            layer_transform: transform,
            pixel_writer,
            drawer,
        }
    }


    pub fn key(&self) -> &str {
        self.key.as_str()
    }


    pub const fn transform_ref(&self) -> &Transform2D {
        &self.layer_transform
    }


    pub fn update_transform<F>(&mut self, fun: F)
    where
        F: FnOnce(&mut Transform2D),
    {
        let transform = &mut self.layer_transform;
        fun(transform);
    }
}


impl<'write, Draw> Layer<EnumPixelWriter, Draw>
where
    Draw: LayerDrawable + 'write,
{
    pub fn draw(&mut self, pixels: &mut [PixelColor]) -> KernelResult {
        self.drawer
            .draw(&self.layer_transform, pixels)
    }


    pub fn draw_in_area(
        &mut self,
        pixels: &mut [PixelColor],
        area: &Rectangle<usize>,
    ) -> KernelResult {
        self.drawer
            .draw_in_area(&self.layer_transform, pixels, area)
    }


    pub fn update_drawer<CastDraw: LayerDrawable>(&mut self, fun: impl Fn(&mut CastDraw)) {
        let drawer = self
            .drawer
            .any_mut()
            .downcast_mut()
            .unwrap();

        fun(drawer);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_success_update_window_position() {}
}
