use alloc::string::String;
use auto_delegate::Delegate;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::transform2d::Transformable2D;

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::multiple_layer::LayerFindable;

#[derive(Delegate)]
pub struct LayerKey {
    key: String,
    #[to(LayerUpdatable, Transformable2D)]
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


    pub fn find_by_key_mut(&mut self, key: &str) -> Option<&mut Layer> {
        if self.key() == key {
            return Some(&mut self.layer);
        }

        match &mut self.layer {
            Layer::Multiple(multi) => multi.find_by_key_mut(key),
            Layer::Window(window) => window.find_by_key_mut(key),
            _ => None,
        }
    }
}


impl LayerKey {
    pub fn update_back_buffer(&mut self, shadow_buff: &mut ShadowFrameBuffer) -> KernelResult {
        self.layer
            .update_back_buffer(shadow_buff, &self.layer.rect())
    }


    pub fn update_back_buffer_in_area(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        area: &Rectangle<usize>,
    ) -> KernelResult {
        if let Some(draw_area) = area.intersect(&self.layer.rect()) {
            self.layer
                .update_back_buffer(shadow_buff, &draw_area)
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
