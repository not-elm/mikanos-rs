use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::{Window, WindowDrawable};

pub struct Layer<'window, Writer> {
    pixel_writer: Writer,
    windows: BTreeMap<&'window str, Window<Box<dyn WindowDrawable>>>,
}


impl<'window, Writer> Layer<'window, Writer> {
    pub const fn new(writer: Writer) -> Self {
        Self {
            pixel_writer: writer,
            windows: BTreeMap::new(),
        }
    }


    pub fn add_window(&mut self, key: &'window str, window: Window<impl WindowDrawable>) {
        self.windows
            .insert(key, window.into_dyn());
    }


    pub fn window_mut_at(
        &'window mut self,
        key: &'window str,
    ) -> Option<&mut Window<Box<dyn WindowDrawable>>> {
        self.windows.get_mut(key)
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
            let pos = window.pos();
            window
                .drawer()
                .draw(pos, &mut self.pixel_writer)?;
        }

        Ok(())
    }
}
