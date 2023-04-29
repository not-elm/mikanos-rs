use alloc::boxed::Box;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;
use crate::layers::drawer::LayerDrawable;

pub struct Layer<'key, Writer, Draw> {
    key: &'key str,
    layer_transform: Transform2D,
    pixel_writer: Writer,
    drawer: Draw, // windows: BTreeMap<&'static str, Window<Box<dyn WindowDrawable>>>,
}


impl<'key, Writer, Draw> Layer<'key, Writer, Draw> {
    pub const fn new(
        key: &'key str,
        transform: Transform2D,
        pixel_writer: Writer,
        drawer: Draw,
    ) -> Self {
        Self {
            key,
            layer_transform: transform,
            pixel_writer,
            drawer,
        }
    }


    pub const fn key(&self) -> &'key str {
        self.key
    }


    pub const fn transform_ref(&self) -> &Transform2D {
        &self.layer_transform
    }


    pub fn update_transform<F>(&mut self, fun: F)
    where
        F: Fn(&mut Transform2D),
    {
        let transform = &mut self.layer_transform;
        fun(transform);
    }
}


impl<'key, Writer> Layer<'key, Writer, Box<dyn LayerDrawable>>
where
    Writer: PixelFlushable + Clone,
{
    pub fn draw(&mut self) -> KernelResult {
        self.drawer
            .draw(&self.layer_transform, &mut self.pixel_writer)
    }


    pub fn draw_in_area(&mut self, area: &Rectangle<usize>) -> KernelResult {
        self.drawer
            .draw_in_area(&self.layer_transform, &mut self.pixel_writer, area)
    }


    pub fn update_drawer<Draw: LayerDrawable>(&mut self, fun: impl FnOnce(&mut Draw)) {
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
    fn it_success_update_window_position() {
        todo!()
    }
}
