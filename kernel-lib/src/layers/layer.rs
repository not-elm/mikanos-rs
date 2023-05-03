use alloc::string::String;
use core::fmt::Debug;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
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
    pub fn draw(&mut self, shadow_buff: &mut [u8]) -> KernelResult {
        self.drawer.draw_in_area(
            shadow_buff,
            &mut self.pixel_writer,
            &self.layer_transform.rect(),
        )
    }


    pub fn write_buff(&mut self, shadow_buff: &mut [u8], area: &Rectangle<usize>) -> KernelResult {
        if let Some(draw_area) = area.intersect(&self.layer_transform.rect()) {
            self.drawer
                .draw_in_area(shadow_buff, &mut self.pixel_writer, &draw_area)
        } else {
            Ok(())
        }
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
