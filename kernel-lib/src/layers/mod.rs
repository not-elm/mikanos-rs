use crate::layers::layer::Layer;
use alloc::vec::Vec;

pub mod layer;
pub mod window;

pub struct Layers<'window, Writer> {
    layers: Vec<Layer<'window, Writer>>,
}


impl<'window, Writer> Layers<'window, Writer> {
    pub const fn new() -> Layers<'window, Writer> {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer<'window, Writer>) {
        self.layers.push(layer);
    }
}
