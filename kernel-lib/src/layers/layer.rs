use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::Window;

pub struct Layer<'window, Writer> {
    pixel_writer: Writer,
    windows: BTreeMap<&'window str, Box<dyn Window + 'window>>,
}


impl<'window, Writer> Layer<'window, Writer> {
    pub const fn new(writer: Writer) -> Self {
        Self {
            pixel_writer: writer,
            windows: BTreeMap::new(),
        }
    }


    pub fn and_add_window(mut self, key: &'window str, window: impl Window + 'window) -> Self {
        self.add_window(key, window);
        self
    }


    pub fn add_window(&mut self, key: &'window str, window: impl Window + 'window) {
        self.windows
            .insert(key, Box::new(window));
    }


    pub fn remove_window(&mut self, key: &str) {
        self.windows.remove(key);
    }
}


impl<'window, Writer> Layer<'window, Writer>
where
    Writer: PixelWritable,
{
    pub fn draw_all(&mut self) -> KernelResult {
        for window in self.windows.values_mut() {
            window.draw(&mut self.pixel_writer)?;
        }

        Ok(())
    }
}
