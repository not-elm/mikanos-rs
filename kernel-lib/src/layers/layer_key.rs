use alloc::string::String;

use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::layer::Layer;

pub struct LayerKey {
    key: String,
    layer: Layer,
}


impl LayerKey {
    pub fn new(key: &str, layer: Layer) -> Self {
        Self {
            key: String::from(key),
            layer,
        }
    }


    pub fn key(&self) -> &str {
        self.key.as_str()
    }
}


impl LayerKey {
    pub fn update_shadow_buffer(&mut self, shadow_buff: &mut ShadowFrameBuffer) -> KernelResult {
        self.layer
            .update_shadow_buffer(shadow_buff, &self.layer.rect())
    }


    pub fn update_shadow_buffer_in_area(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        area: &Rectangle<usize>,
    ) -> KernelResult {
        if let Some(draw_area) = area.intersect(&self.layer.rect()) {
            self.layer
                .update_shadow_buffer(shadow_buff, &draw_area)
        } else {
            Ok(())
        }
    }


    pub fn layer_ref(&self) -> &Layer {
        &self.layer
    }


    pub fn layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }


    pub fn update_layer(&mut self, fun: impl Fn(&mut Layer)) {
        fun(&mut self.layer);
    }
}
