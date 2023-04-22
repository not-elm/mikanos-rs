use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::gop::pixel::rc_pixel_writer::RcPixelWriter;
use crate::layers::layer::Layer;
use crate::layers::layer_status::LayerStatus;

pub mod layer;
pub mod layer_status;
pub mod window;


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


    pub fn new_layer(&mut self, layer_status: LayerStatus) -> &mut Layer<RcPixelWriter<'window>> {
        self.layers
            .push(Layer::new(layer_status, self.writer.clone()));

        self.layers
            .last_mut()
            .unwrap()
    }
}
