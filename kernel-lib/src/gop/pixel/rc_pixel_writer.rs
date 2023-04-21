use alloc::rc::Rc;
use core::cell::RefCell;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub struct RcPixelWriter<'window>(Rc<RefCell<dyn PixelWritable + 'window>>);


impl<'writer> RcPixelWriter<'writer> {
    pub fn new(writer: impl PixelWritable + 'writer) -> RcPixelWriter<'writer> {
        Self(Rc::new(RefCell::new(writer)))
    }


    pub fn from_rc(writer: Rc<RefCell<dyn PixelWritable + 'writer>>) -> RcPixelWriter<'writer> {
        Self(writer)
    }
}


impl<'writer> PixelWritable for RcPixelWriter<'writer> {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        let mut writer = self.0.borrow_mut();
        writer.write(x, y, color)
    }
}


impl<'writer> Clone for RcPixelWriter<'writer> {
    fn clone(&self) -> Self {
        RcPixelWriter(Rc::clone(&self.0))
    }
}
