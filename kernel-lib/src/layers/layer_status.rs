use crate::error::{KernelError, KernelResult, LayerReason};
use crate::layers::window::status::WindowStatus;
use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

#[derive(Debug, Clone)]
pub struct LayerStatus {
    size: Size,
}


impl LayerStatus {
    pub const fn new(container_size: Size) -> LayerStatus {
        Self {
            size: container_size,
        }
    }


    pub fn with_in_rect(&self, rect: Rectangle<usize>) -> bool {
        let layer_rect = Rectangle::from_size(Vector2D::zeros(), self.size);

        layer_rect.with_in_rect(&rect)
    }
}


pub struct LayerStatusBuilder {
    size: Size,
}

impl LayerStatusBuilder {
    pub fn new(size: Size) -> Self {
        Self { size }
    }


    pub fn build(self) -> LayerStatus {
        LayerStatus::new(self.size)
    }
}


pub fn check_window_status(
    layer_status: &LayerStatus,
    window_status: &WindowStatus,
) -> KernelResult {
    if layer_status.with_in_rect(window_status.window_rect()) {
        Ok(())
    } else {
        Err(KernelError::FailedOperateLayer(
            LayerReason::WindowSizeOver(window_status.window_rect()),
        ))
    }
}
