use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt::Write;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::Transform2D;

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;
use crate::gop::pixel::writer::rc_pixel_writer::RcPixelWriter;
use crate::layers::drawer::LayerDrawable;
use crate::layers::layer::Layer;

pub mod drawer;
pub mod layer;
pub mod window;


pub fn frame_buffer_layer_transform(frame_buffer_config: FrameBufferConfig) -> Transform2D {
    Transform2DBuilder::new()
        .size(frame_buffer_config.frame_size())
        .build()
}


pub struct Layers<'window> {
    writer: RcPixelWriter<'window>,
    layers: Vec<Layer<'window, RcPixelWriter<'window>, Box<dyn LayerDrawable>>>,
}


impl<'window> Layers<'window> {
    pub fn new(writer: impl PixelFlushable + 'window) -> Layers<'window> {
        Self {
            writer: RcPixelWriter::new(writer),
            layers: Vec::new(),
        }
    }


    pub fn new_with_rc(writer: Rc<RefCell<dyn PixelFlushable + 'window>>) -> Layers<'window> {
        Self {
            writer: RcPixelWriter::from_rc(writer),
            layers: Vec::new(),
        }
    }


    pub fn new_layer(
        &mut self,
        key: &'window str,
        transform: Transform2D,
        drawer: impl LayerDrawable,
    ) {
        self.layers.push(Layer::new(
            key,
            transform,
            self.writer.clone(),
            Box::new(drawer),
        ));
    }


    pub fn update_layer(
        &mut self,
        key: &'window str,
        fun: impl FnOnce(&mut Layer<'window, RcPixelWriter<'window>, Box<dyn LayerDrawable>>),
    ) -> KernelResult {
        let prev_rect = self
            .layer_ref(key)?
            .transform_ref()
            .rect();

        fun(self.layer_mut(key)?);

        self.draw_all_layer_in_area(Some(key), &prev_rect)?;

        self.draw_all_layer_in_area(
            None,
            &self
                .layer_ref(key)?
                .transform_ref()
                .rect(),
        )
    }


    pub fn draw_all_layer(&mut self) -> KernelResult {
        for layer in self.layers.iter_mut() {
            layer.draw()?;
        }

        Ok(())
    }


    pub fn draw_all_layer_in_area(
        &mut self,
        key: Option<&str>,
        area: &Rectangle<usize>,
    ) -> KernelResult {
        for layer in self
            .layers
            .iter_mut()
            .take_while(|layer| key.map_or(true, |key| key != layer.key()))
        {
            layer.draw_in_area(area)?;
        }

        Ok(())
    }


    fn layer_ref(
        &self,
        key: &'window str,
    ) -> Result<&Layer<'window, RcPixelWriter<'window>, Box<dyn LayerDrawable>>, KernelError> {
        self.layers
            .iter()
            .find(|layer| layer.key() == key)
            .ok_or(KernelError::FailedOperateLayer(LayerReason::NotExistsKey))
    }


    fn layer_mut(
        &mut self,
        key: &'window str,
    ) -> Result<&mut Layer<'window, RcPixelWriter<'window>, Box<dyn LayerDrawable>>, KernelError>
    {
        self.layers
            .iter_mut()
            .find(|layer| layer.key() == key)
            .ok_or(KernelError::FailedOperateLayer(LayerReason::NotExistsKey))
    }
}
