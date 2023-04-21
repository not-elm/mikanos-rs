use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::gop::pixel::rc_pixel_writer::RcPixelWriter;
use crate::layers::layer::Layer;

pub mod layer;
pub mod window;


pub struct Layers<'window> {
    writer: RcPixelWriter<'window>,
    layers: Vec<Layer<'window, RcPixelWriter<'window>>>,
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


    pub fn layer_mut_at(
        &'window mut self,
        index: usize,
    ) -> Option<&'window mut Layer<'window, RcPixelWriter<'window>>> {
        self.layers.get_mut(index)
    }


    pub fn new_layer(&mut self) -> &mut Layer<'window, RcPixelWriter<'window>> {
        self.layers
            .push(Layer::new(self.writer.clone()));
        self.layers
            .last_mut()
            .unwrap()
    }
}
