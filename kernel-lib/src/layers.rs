use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::any::Any;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::Transform2D;

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::enum_pixel_writer::EnumPixelWriter;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
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


pub struct DynLayerDrawer(Box<dyn LayerDrawable>);


impl DynLayerDrawer {
    pub fn new(drawer: impl LayerDrawable) -> DynLayerDrawer {
        Self(Box::new(drawer))
    }
}


impl LayerDrawable for DynLayerDrawer {
    fn draw_in_area(
        &mut self,
        transform: &Transform2D,
        pixels: &mut [PixelColor],
        draw_rect: &Rectangle<usize>,
    ) -> KernelResult {
        self.0
            .draw_in_area(transform, pixels, draw_rect)
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self.0.any_mut()
    }
}


pub struct Layers {
    frame_buffer_config: FrameBufferConfig,
    writer: EnumPixelWriter,
    pixels: Vec<PixelColor>,
    layers: Vec<Layer<EnumPixelWriter, DynLayerDrawer>>,
}


impl Layers {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Layers {
        Self {
            writer: EnumPixelWriter::new(frame_buffer_config),
            pixels: vec![
                PixelColor::black();
                frame_buffer_config.horizontal_resolution
                    * frame_buffer_config.vertical_resolution
            ],
            layers: Vec::new(),
            frame_buffer_config,
        }
    }


    pub fn new_layer(&mut self, key: &str, transform: Transform2D, drawer: impl LayerDrawable) {
        self.layers.push(Layer::new(
            key,
            transform,
            self.writer.clone(),
            DynLayerDrawer::new(drawer),
        ));
    }


    pub fn update_layer(
        &mut self,
        key: &str,
        fun: impl FnOnce(&mut Layer<EnumPixelWriter, DynLayerDrawer>),
    ) -> KernelResult {
        let prev_area = self
            .layer_ref(key)?
            .transform_ref()
            .rect();

        fun(self.layer_mut(key)?);


        // self.draw_all_layer_in_area(&prev_area)?;

        let draw_area = &self
            .layer_ref(key)?
            .transform_ref()
            .rect();
        self.draw_all_layer_in_area(draw_area)
        // self.draw_all_layer_in_area(MOUSE_LAYER_KEY, None, draw_area)
    }


    pub fn draw_all_layer(&mut self) -> KernelResult {
        for layer in self.layers.iter_mut() {
            layer.draw(self.pixels.as_mut_slice())?;
        }
        self.draw(&Rectangle::from_size(
            self.frame_buffer_config
                .frame_size(),
        ))
    }


    pub fn draw_all_layer_in_area(&mut self, area: &Rectangle<usize>) -> KernelResult {
        // let mut pixels = vec![PixelColor::black(); area.width() * area.height()];
        let layer = self
            .layers
            .iter_mut()
            .last()
            .unwrap();
        {
            layer.draw_in_area(self.pixels.as_mut_slice(), area)?;
        }

        self.draw(area)?;

        Ok(())
    }


    fn draw(&mut self, area: &Rectangle<usize>) -> KernelResult {
        for y in 0..area.height() {
            for x in 0..area.width() {
                let ix = x + area.origin().x();
                let iy = y + area.origin().y();

                unsafe {
                    self.writer
                        .write(ix, iy, &self.pixels[x + (y * area.width())])?;
                }
            }
        }


        Ok(())
    }


    fn layer_ref(&self, key: &str) -> KernelResult<&Layer<EnumPixelWriter, DynLayerDrawer>> {
        self.layers
            .iter()
            .find(|layer| layer.key() == key)
            .ok_or(KernelError::FailedOperateLayer(LayerReason::NotExistsKey))
    }


    fn layer_mut(
        &mut self,
        key: &str,
    ) -> KernelResult<&mut Layer<EnumPixelWriter, DynLayerDrawer>> {
        self.layers
            .iter_mut()
            .find(|layer| layer.key() == key)
            .ok_or(KernelError::FailedOperateLayer(LayerReason::NotExistsKey))
    }
}
