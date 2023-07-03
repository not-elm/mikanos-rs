use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::OnceCell;

use spin::MutexGuard;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::vector::Vector2D;
use common_lib::transform::builder::Transform2DBuilder;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::kernel_error;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::sync::preemptive_mutex::PreemptiveMutex;

pub mod close_button;
pub mod count;
pub mod cursor;
pub mod layer;
pub mod layer_key;
pub mod layer_updatable;
pub mod multiple_layer;
pub mod plain;
pub mod shape;
pub mod terminal;
pub mod text;
pub mod text_box;
pub mod window;


pub static LAYERS: GlobalLayers = GlobalLayers::new_uninit();

pub struct GlobalLayers(OnceCell<PreemptiveMutex<Layers>>);


impl GlobalLayers {
    pub const fn new_uninit() -> GlobalLayers {
        Self(OnceCell::new())
    }

    pub fn init(&self, frame_buffer_config: FrameBufferConfig) -> KernelResult {
        let layers = Layers::new(frame_buffer_config);

        self.0
            .set(PreemptiveMutex::new(layers))
            .map_err(|_| KernelError::FailedOperateLayer(LayerReason::FailedInitialize))
    }


    pub fn lock(&self) -> MutexGuard<Layers> {
        self.0.get().unwrap().lock()
    }
}


unsafe impl Sync for GlobalLayers {}


pub fn frame_buffer_layer_transform(frame_buffer_config: FrameBufferConfig) -> Transform2D {
    Transform2DBuilder::new()
        .size(frame_buffer_config.screen_size())
        .build()
}


pub struct Layers {
    frame_buffer_config: FrameBufferConfig,
    back_buffer: ShadowFrameBuffer,
    layers: Vec<LayerKey>,
}


impl Layers {
    #[inline(always)]
    pub fn new(frame_buffer_config: FrameBufferConfig) -> Layers {
        Self {
            back_buffer: ShadowFrameBuffer::new(frame_buffer_config),
            layers: Vec::new(),
            frame_buffer_config,
        }
    }


    #[inline(always)]
    pub fn new_layer(&mut self, layer_key: LayerKey) {
        self.layers.push(layer_key);
    }


    pub fn bring_to_front(&mut self, key: &str) -> KernelResult {
        let index = self
            .index_by_key(key)
            .ok_or(kernel_error!("Not found key = {}", key))?;

        let layer = self.layers.remove(index);
        self.layers.push(layer);

        Ok(())
    }


    #[inline]
    pub fn find_window_layer_by_pos(&self, pos: &Vector2D<usize>) -> Option<&str> {
        self.layers
            .iter()
            .filter(|layer| layer.rect().with_in_pos(pos))
            .filter(|layer| layer.layer_ref().is_window())
            .map(|layer| layer.key())
            .last()
    }


    pub fn update_active_layer(&mut self, fun: impl FnOnce(&mut Layer)) -> KernelResult {
        if let Some(key) = self.find_active_window_layer_ref() {
            let key = &key.key().to_string();
            self.update_layer(key, fun)?;
        }

        Ok(())
    }


    pub fn update_layer(&mut self, key: &str, fun: impl FnOnce(&mut Layer)) -> KernelResult {
        let prev = self
            .layer_ref(key)?
            .transform();

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
            layer.update_back_buffer(&mut self.back_buffer)?;
        }

        self.flush(&Rectangle::from_size(
            self.frame_buffer_config
                .screen_size(),
        ))
    }


    #[inline(always)]
    fn index_by_key(&self, key: &str) -> Option<usize> {
        self.layers
            .iter()
            .position(|layer| layer.key() == key)
    }


    #[inline]
    pub fn find_active_window_layer_ref(&self) -> Option<&LayerKey> {
        self.layers
            .iter()
            .find(|layer| layer.is_active_window())
    }


    #[inline]
    pub fn find_active_window_layer_mut(&mut self) -> Option<&mut LayerKey> {
        self.layers
            .iter_mut()
            .find(|layer| layer.is_active_window())
    }


    fn draw_from_at(&mut self, key: &str, prev_area: &Rectangle<usize>) -> KernelResult {
        self.update_back_buffer_in_area(prev_area, None, Some(key))?;

        let draw_area = &self.layer_ref(key)?.rect();

        self.update_back_buffer_in_area(draw_area, Some(key), None)?;
        self.flush(&prev_area.union(draw_area))
    }


    fn update_back_buffer_in_area(
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

            if let Some(draw_rect) = area.intersect(&layer.rect()) {
                layer.update_back_buffer_in_area(&mut self.back_buffer, &draw_rect)?;
            }
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
            self.back_buffer.raw_ref(),
            frame_buffer,
            &self.frame_buffer_config,
            area,
        )
    }


    pub fn deactivate(&mut self) -> KernelResult {
        // if !(layer.is_active_window() && layer.key() != current_active_key) {
        //     return Ok(());
        // }

        if let Some(window) = self.find_active_window_layer_mut() {
            window
                .layer_mut()
                .require_window()
                .unwrap()
                .deactivate()?;

            let draw_area = &window.layer_ref().rect();
            let key = window.key().to_string();

            self.update_back_buffer_in_area(draw_area, Some(&key), None)?;
        }

        Ok(())
    }


    #[inline(always)]
    fn frame_rect(&self) -> Rectangle<usize> {
        self.frame_buffer_config
            .frame_rect()
    }


    #[inline]
    fn layer_ref(&self, key: &str) -> KernelResult<&Layer> {
        self.layers
            .iter()
            .find_map(|layer| layer.find_by_key(key))
            .ok_or(kernel_error!("Not exists key = {}", key))
    }


    #[inline]
    fn layer_mut(&mut self, key: &str) -> KernelResult<&mut Layer> {
        self.layers
            .iter_mut()
            .find_map(|layer| layer.find_by_key_mut(key))
            .ok_or(kernel_error!("Not exists key = {}", key))
    }
}


#[inline(always)]
pub(crate) fn copy_frame_buff_in_area(
    src: &[u8],
    dist: &mut [u8],
    _config: &FrameBufferConfig,
    _area: &Rectangle<usize>,
) -> KernelResult {
    dist.copy_from_slice(src);

    Ok(())
}
