use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use common_lib::{frame_buffer::FrameBufferConfig, transform::transform2d::Transformable2D};

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::kernel_error;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::MultipleLayer;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::window::toolbar::ToolbarLayer;

pub(crate) mod toolbar;

const TOOLBAR_HEIGHT: usize = 24;
const TOOLBAR_LAYER_KEY: &str = "Window Toolbar";


#[derive(Delegate)]
pub struct WindowLayer {
    active: bool,

    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    layers: MultipleLayer,
}


impl WindowLayer {
    pub fn new(config: FrameBufferConfig, transform: Transform2D, title: &str) -> Self {
        let mut multiple_layer = MultipleLayer::new(transform.clone());

        multiple_layer.new_layer(shadow_layer(config, &transform));
        multiple_layer.new_layer(window_background_layer(config, &transform));
        multiple_layer.new_layer(toolbar_layer(config, &transform, title));

        Self {
            active: false,
            layers: multiple_layer,
        }
    }


    pub fn new_layer(mut self, mut layer: LayerKey) -> KernelResult<Self> {
        layer
            .move_to_relative(Vector2D::new(0, TOOLBAR_HEIGHT as isize + 5))
            .map_err(|e| kernel_error!(e))?;

        self.layers.new_layer(layer);
        Ok(self)
    }


    pub fn activate(&mut self) -> KernelResult {
        self.layers
            .force_find_by_key_mut(TOOLBAR_LAYER_KEY)
            .require_toolbar()
            .unwrap()
            .activate()?;

        self.active = true;
        Ok(())
    }


    pub fn deactivate(&mut self) -> KernelResult {
        self.layers
            .force_find_by_key_mut(TOOLBAR_LAYER_KEY)
            .require_toolbar()
            .unwrap()
            .deactivate()?;

        self.active = false;
        Ok(())
    }


    #[inline(always)]
    pub const fn into_enum(self) -> Layer {
        Layer::Window(self)
    }


    #[inline(always)]
    pub const fn is_active(&self) -> bool {
        self.active
    }
}


fn shadow_layer(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
    ShapeLayer::new(
        ShapeDrawer::new(config, PixelColor::black()),
        Transform2D::new(Vector2D::zeros(), transform.size()),
    )
    .into_enum()
    .into_layer_key("window shadow")
}


fn window_background_layer(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
    ShapeLayer::new(
        ShapeDrawer::new(config, PixelColor::window_background()),
        Transform2D::new(
            Vector2D::zeros(),
            Size::new(transform.size().width() - 1, transform.size().height() - 1),
        ),
    )
    .into_enum()
    .into_layer_key("window background")
}


fn toolbar_layer(config: FrameBufferConfig, transform: &Transform2D, title: &str) -> LayerKey {
    let toolbar_transform = Transform2D::new(
        Vector2D::new(1, 1),
        Size::new(transform.size().width() - 3, TOOLBAR_HEIGHT),
    );

    ToolbarLayer::new(config, toolbar_transform, title)
        .into_enum()
        .into_layer_key(TOOLBAR_LAYER_KEY)
}


#[cfg(test)]
mod tests {}
