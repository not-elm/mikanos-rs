use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::layer::Layer;

pub mod layer;
pub mod window;

pub struct RcWriter<'window>(Rc<RefCell<dyn PixelWritable + 'window>>);

impl<'writer> RcWriter<'writer> {
    pub fn new(writer: impl PixelWritable + 'writer) -> RcWriter<'writer> {
        Self(Rc::new(RefCell::new(writer)))
    }
}

impl<'writer> PixelWritable for RcWriter<'writer> {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        let mut writer = self.0.borrow_mut();
        writer.write(x, y, color)
    }
}


impl<'writer> Clone for RcWriter<'writer> {
    fn clone(&self) -> Self {
        RcWriter(Rc::clone(&self.0))
    }
}

pub struct Layers<'window> {
    writer: RcWriter<'window>,
    layers: Vec<Layer<'window, RcWriter<'window>>>,
}


impl<'window> Layers<'window> {
    pub fn new(writer: impl PixelWritable + 'window) -> Layers<'window> {
        Self {
            writer: RcWriter::new(writer),
            layers: Vec::new(),
        }
    }


    pub fn new_with_rc(writer: Rc<RefCell<dyn PixelWritable + 'window>>) -> Layers<'window> {
        Self {
            writer: RcWriter(writer),
            layers: Vec::new(),
        }
    }

    pub fn at(&'window mut self, index: usize) -> Option<&'window mut Layer<'window, RcWriter<'window>>> {
        self.layers.get_mut(index)
    }

    pub fn new_layer(&mut self) -> &mut Layer<'window, RcWriter<'window>> {
        self.layers.push(Layer::new(self.writer.clone()));
        self.layers.last_mut().unwrap()
    }
}
