use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::gop::pixel::rc_pixel_writer::RcPixelWriter;
use crate::layers::layer::Layer;

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
    pub fn new(writer: impl PixelWritable + 'window) -> Layers<'window> {
        Self {
            writer: RcPixelWriter::new(writer),
            layers: Vec::new(),
        }
    }


    pub fn new_with_rc(writer: Rc<RefCell<dyn PixelWritable + 'window>>) -> Layers<'window> {
        Self {
            writer: RcPixelWriter::from_rc(writer),
            layers: Vec::new(),
        }
    }


    pub fn layer_mut_at(&mut self, index: usize) -> &mut Layer<RcPixelWriter<'window>> {
        self.layers
            .get_mut(index)
            .unwrap()
    }


    pub fn new_layer(&mut self, transform: Transform2D) -> &mut Layer<RcPixelWriter<'window>> {
        self.layers
            .push(Layer::new(transform, self.writer.clone()));

        self.layers
            .last_mut()
            .unwrap()
    }


    pub fn draw_all_layers_start_at(&mut self, index: usize) -> KernelResult {
        for layer in self
            .layers
            .iter_mut()
            .skip(index)
        {
            layer.draw_all_window()?;
        }

        Ok(())
    }
}
