use alloc::vec::Vec;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::vector::Vector2D;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::gop::pixel::calc_pixel_pos;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;

pub mod console;
pub mod cursor;
pub mod layer;
pub mod layer_key;
pub mod layer_updatable;
pub mod multiple_layer;
pub mod plain;
pub mod shape;
pub mod window;


pub fn frame_buffer_layer_transform(frame_buffer_config: FrameBufferConfig) -> Transform2D {
    Transform2DBuilder::new()
        .size(frame_buffer_config.frame_size())
        .build()
}


pub struct Layers {
    frame_buffer_config: FrameBufferConfig,
    shadow_buffer: ShadowFrameBuffer,
    layers: Vec<LayerKey>,
}


impl Layers {
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Layers {
        Self {
            shadow_buffer: ShadowFrameBuffer::new(frame_buffer_config),
            layers: Vec::new(),
            frame_buffer_config,
        }
    }


    pub fn new_layer(&mut self, layer_key: LayerKey) {
        self.layers.push(layer_key);
    }


    pub fn find_window_layer_by_pos(&self, pos: &Vector2D<usize>) -> Option<&str> {
        self.layers
            .iter()
            .filter(|layer| layer.rect().with_in_pos(pos))
            .filter(|layer| layer.layer_ref().is_window())
            .map(|layer| layer.key())
            .last()
    }


    pub fn update_layer(&mut self, key: &str, fun: impl FnOnce(&mut Layer)) -> KernelResult {
        let prev = self
            .layer_ref(key)?
            .transform_ref()
            .clone();

        let frame_rect = self.frame_rect();
        let layer = self.layer_mut(key)?;
        fun(layer);

        if !frame_rect.with_in_rect(&layer.rect()) {
            layer.move_to(prev.pos());
            return Ok(());
        }

        self.draw_from_at(key, &prev.rect())
    }


    pub fn draw_all_layer(&mut self) -> KernelResult {
        for layer in self.layers.iter_mut() {
            layer.update_shadow_buffer(&mut self.shadow_buffer)?;
        }

        self.flush(&Rectangle::from_size(
            self.frame_buffer_config
                .frame_size(),
        ))
    }


    fn draw_from_at(&mut self, key: &str, prev_area: &Rectangle<usize>) -> KernelResult {
        self.update_shadow_buffer_in_area(prev_area, None, Some(key))?;

        let draw_area = &self.layer_ref(key)?.rect();

        self.update_shadow_buffer_in_area(draw_area, Some(key), None)?;

        self.flush(&prev_area.union(draw_area))
    }


    fn update_shadow_buffer_in_area(
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

            layer.update_shadow_buffer_in_area(&mut self.shadow_buffer, area)?;
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

        copy_frame_buff_in_area(
            self.shadow_buffer.raw_ref(),
            frame_buffer,
            &self.frame_buffer_config,
            area,
        )
    }


    fn frame_rect(&self) -> Rectangle<usize> {
        self.frame_buffer_config
            .frame_rect()
    }


    fn layer_ref(&self, key: &str) -> KernelResult<&Layer> {
        Ok(self
            .layers
            .iter()
            .find(|layer| layer.key() == key)
            .ok_or(KernelError::FailedOperateLayer(LayerReason::NotExistsKey))?
            .layer_ref())
    }


    fn layer_mut(&mut self, key: &str) -> KernelResult<&mut Layer> {
        Ok(self
            .layers
            .iter_mut()
            .find(|layer| layer.key() == key)
            .ok_or(KernelError::FailedOperateLayer(LayerReason::NotExistsKey))?
            .layer_mut())
    }
}


pub(crate) fn copy_frame_buff_in_area(
    src: &[u8],
    dist: &mut [u8],
    config: &FrameBufferConfig,
    area: &Rectangle<usize>,
) -> KernelResult {
    for y in area.origin().y()..area.end().y() {
        let origin = calc_pixel_pos(config, area.origin().x(), y)?;
        let end = calc_pixel_pos(config, area.end().x(), y)?;

        dist[origin..end].copy_from_slice(&src[origin..end]);
    }

    Ok(())
}
