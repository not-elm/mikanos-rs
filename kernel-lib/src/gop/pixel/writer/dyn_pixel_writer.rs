use alloc::rc::Rc;
use core::cell::RefCell;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;

pub struct DynPixelWriter<'write>(Rc<RefCell<dyn PixelWritable + 'write>>);


impl<'writer> DynPixelWriter<'writer> {
    pub fn new(writer: impl PixelWritable + 'writer) -> DynPixelWriter<'writer> {
        Self(Rc::new(RefCell::new(writer)))
    }


    pub fn from_rc(writer: Rc<RefCell<dyn PixelWritable + 'writer>>) -> DynPixelWriter<'writer> {
        Self(writer)
    }
}


impl<'writer> PixelWritable for DynPixelWriter<'writer> {
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult {
        self.0
            .borrow_mut()
            .write(x, y, color)
    }


    unsafe fn write_shadow_buff(
        &mut self,
        buff: &mut [u8],
        x: usize,
        y: usize,
        color: &PixelColor,
    ) -> KernelResult {
        self.0
            .borrow_mut()
            .write_shadow_buff(buff, x, y, color)
    }
}


// impl<'writer> PixelFlushable for RcPixelWriter<'writer> {
//     unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult {
//         self.0
//             .borrow_mut()
//             .flush(pixel_frame)
//     }
// }


impl<'writer> Clone for DynPixelWriter<'writer> {
    fn clone(&self) -> Self {
        DynPixelWriter(Rc::clone(&self.0))
    }
}
