use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::any::Any;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::Transform2D;

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::pixel::writer::enum_pixel_writer::EnumPixelWriter;
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
        pixels: &mut [u8],
        pixel_writer: &mut EnumPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        self.0
            .draw_in_area(pixels, pixel_writer, draw_area)
    }


    fn any_mut(&mut self) -> &mut dyn Any {
        self.0.any_mut()
    }
}


pub struct Layers {
    frame_buffer_config: FrameBufferConfig,
    writer: EnumPixelWriter,
    shadow_buffer: Vec<u8>,
    layers: Vec<Layer<EnumPixelWriter, DynLayerDrawer>>,
}


impl Layers {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Layers {
        let pixels = vec![0; frame_buffer_config.frame_buff_length()];


        Self {
            writer: EnumPixelWriter::new(frame_buffer_config),
            shadow_buffer: pixels,
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


        self.write_shadow_buffer_in_area(&prev_area, None, Some(key))?;

        let draw_area = &self
            .layer_ref(key)?
            .transform_ref()
            .rect();
        self.write_shadow_buffer_in_area(draw_area, Some(key), None)?;
        self.flush(&prev_area.union(draw_area))
    }


    pub fn draw_all_layer(&mut self) -> KernelResult {
        for layer in self.layers.iter_mut() {
            layer.draw(
                self.shadow_buffer
                    .as_mut_slice(),
            )?;
        }


        self.flush(&Rectangle::from_size(
            self.frame_buffer_config
                .frame_size(),
        ))
    }


    fn write_shadow_buffer_in_area(
        &mut self,
        area: &Rectangle<usize>,
        start_key: Option<&str>,
        end_key: Option<&str>,
    ) -> KernelResult {
        for layer in self
            .layers
            .iter_mut()
            .skip_while(|layer| start_key.map_or(false, |key| key != layer.key()))
        {
            if end_key.is_some_and(|end_key| end_key == layer.key()) {
                return Ok(());
            }


            layer.write_buff(
                self.shadow_buffer
                    .as_mut_slice(),
                area,
            )?;
        }


        Ok(())
    }


    fn flush(&mut self, area: &Rectangle<usize>) -> KernelResult {
        let frame_buffer = unsafe {
            core::slice::from_raw_parts_mut(
                self.frame_buffer_config
                    .frame_buffer_base_ptr(),
                self.frame_buffer_config
                    .frame_buff_length(),
            )
        };
        for y in area.origin().y()..area.end().y() {
            let origin = calc_pixel_pos(&self.frame_buffer_config, area.origin().x(), y)?;
            let end = calc_pixel_pos(&self.frame_buffer_config, area.end().x(), y)?;

            frame_buffer[origin..end].copy_from_slice(&self.shadow_buffer[origin..end]);
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
