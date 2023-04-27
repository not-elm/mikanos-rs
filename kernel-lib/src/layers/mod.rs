use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;
use crate::gop::pixel::writer::rc_pixel_writer::RcPixelWriter;
use crate::layers::layer::Layer;
use crate::layers::window::drawers::WindowDrawable;
use crate::layers::window::Window;

pub mod layer;
pub mod window;


pub fn frame_buffer_layer_transform(frame_buffer_config: FrameBufferConfig) -> Transform2D {
    Transform2DBuilder::new()
        .size(frame_buffer_config.frame_size())
        .build()
}


pub struct Layers<'window> {
    writer: RcPixelWriter<'window>,
    layers: Vec<Layer<RcPixelWriter<'window>>>,
}


impl<'window> Layers<'window> {
    pub fn new(writer: impl PixelFlushable + 'window) -> Layers<'window> {
        Self {
            writer: RcPixelWriter::new(writer),
            layers: Vec::new(),
        }
    }


    pub fn new_with_rc(writer: Rc<RefCell<dyn PixelFlushable + 'window>>) -> Layers<'window> {
        Self {
            writer: RcPixelWriter::from_rc(writer),
            layers: Vec::new(),
        }
    }

    pub fn find_child_window_transform(&self, index: usize, key: &'static str) -> KernelResult<Transform2D>{
        self.find_child_window(index, key)
            .map(|window| window.transform_ref().clone())
    }

    pub fn find_child_window(&self, index: usize, key: &'static str) -> KernelResult<&Window<Box<dyn WindowDrawable>>> {
        self.layer_ref_at(index)
            .window_ref(key)
    }


    pub fn layer_mut_at(&mut self, index: usize) -> &mut Layer<RcPixelWriter<'window>> {
        self.layers
            .get_mut(index)
            .unwrap()
    }


    pub fn layer_ref_at(&self, index: usize) -> &Layer<RcPixelWriter<'window>> {
        self.layers
            .get(index)
            .unwrap()
    }

    pub fn new_layer(&mut self, transform: Transform2D) -> &mut Layer<RcPixelWriter<'window>> {
        self.layers
            .push(Layer::new(transform, self.writer.clone()));

        self.layers
            .last_mut()
            .unwrap()
    }


    pub fn draw_all_layers(&mut self, start_index: usize) -> KernelResult {
        for layer in self
            .layers
            .iter_mut()
            .skip(start_index)
        {
            layer.draw_all_window()?;
        }

        Ok(())
    }


    pub fn draw_all_layers_in_area(
        &mut self,
        start_index: usize,
        draw_rect: &Rectangle<usize>,
    ) -> KernelResult {
        for layer in self
            .layers
            .iter_mut()
            .skip(start_index)
        {
            layer.draw_all_window_in_area(draw_rect)?;
        }

        Ok(())
    }


    pub fn draw_all_layers_until(
        &mut self,
        start_index: usize,
        draw_layers_count: usize,
        draw_rect: &Rectangle<usize>,
    ) -> KernelResult {
        for layer in self
            .layers
            .iter_mut()
            .skip(start_index)
            .take(draw_layers_count)
        {
            layer.draw_all_window_in_area(draw_rect)?;
        }

        Ok(())
    }
}
