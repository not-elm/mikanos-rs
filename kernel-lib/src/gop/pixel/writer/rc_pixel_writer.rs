use alloc::rc::Rc;
use core::cell::RefCell;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;

pub struct RcPixelWriter<'window>(Rc<RefCell<dyn PixelFlushable + 'window>>);


impl<'writer> RcPixelWriter<'writer> {
    pub fn new(writer: impl PixelFlushable + 'writer) -> RcPixelWriter<'writer> {
        Self(Rc::new(RefCell::new(writer)))
    }


    pub fn from_rc(writer: Rc<RefCell<dyn PixelFlushable + 'writer>>) -> RcPixelWriter<'writer> {
        Self(writer)
    }
}


impl<'writer> PixelFlushable for RcPixelWriter<'writer> {
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
        self.0
            .borrow_mut()
            .flush(pixel_frame)
    }
}


impl<'writer> Clone for RcPixelWriter<'writer> {
    fn clone(&self) -> Self {
        RcPixelWriter(Rc::clone(&self.0))
    }
}
