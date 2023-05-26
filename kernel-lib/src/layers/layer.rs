use auto_delegate::Delegate;

use common_lib::transform::transform2d::Transformable2D;

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::layers::console::ConsoleLayer;
use crate::layers::count::CountLayer;
use crate::layers::cursor::CursorLayer;
use crate::layers::layer_key::LayerKey;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::ShapeLayer;
use crate::layers::window::WindowLayer;

use super::close_button::CloseButtonLayer;
use super::multiple_layer::MultipleLayer;

#[derive(Delegate)]
#[to(Transformable2D, LayerUpdatable)]
pub enum Layer {
    Cursor(CursorLayer),
    Console(ConsoleLayer),
    Shape(ShapeLayer),
    Window(WindowLayer),
    CloseButton(CloseButtonLayer),
    Multiple(MultipleLayer),
    Count(CountLayer),
}


impl Layer {
    pub fn require_cursor(&mut self) -> KernelResult<&mut CursorLayer> {
        match self {
            Self::Cursor(cursor) => Ok(cursor),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    pub fn require_console(&mut self) -> KernelResult<&mut ConsoleLayer> {
        match self {
            Self::Console(console) => Ok(console),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    pub fn require_window(&mut self) -> KernelResult<&mut WindowLayer> {
        match self {
            Self::Window(window) => Ok(window),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    pub fn require_count(&mut self) -> KernelResult<&mut CountLayer> {
        match self {
            Self::Count(count) => Ok(count),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    pub fn is_window(&self) -> bool {
        matches!(self, Self::Window(_))
    }


    pub fn into_layer_key(self, key: &str) -> LayerKey {
        LayerKey::new(key, self)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_success_update_window_position() {}
}
