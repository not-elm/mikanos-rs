use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::Window;

pub struct Layer<'window, Writer> {
    id: usize,
    pixel_writer: Writer,
    windows: BTreeMap<&'window str, Box<dyn Window + 'window>>,
}


impl<'window, Writer> Layer<'window, Writer> {
    pub fn new(id: usize, writer: Writer) -> Self {
        Self {
            id,
            pixel_writer: writer,
            windows: BTreeMap::new(),
        }
    }
    
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn and_add_window(mut self, key: &'window str, window: impl Window + 'window) -> Self {
        self.add_window(key, window);
        self
    }

    pub fn add_window(&mut self, key: &'window str, window: impl Window + 'window) {
        self.windows.insert(key, Box::new(window));
    }

    pub fn remove_window(&mut self, key: &str) {
        self.windows.remove(key);
    }
}

impl<'window, Writer> Layer<'window, Writer> where Writer: PixelWritable {
    pub fn draw_all(&mut self) {
        self
            .windows
            .values_mut()
            .for_each(|window| {
                window.draw(&mut self.pixel_writer)
            });
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_() {}
}