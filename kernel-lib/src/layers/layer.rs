use core::fmt::{Display, Formatter};

use auto_delegate::Delegate;

use common_lib::transform::transform2d::Transformable2D;

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::kernel_error;
use crate::layers::count::CountLayer;
use crate::layers::cursor::CursorLayer;
use crate::layers::layer_key::LayerKey;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::TextLayer;
use crate::layers::text_box::TextBoxLayer;
use crate::layers::window::toolbar::ToolbarLayer;
use crate::layers::window::WindowLayer;

use super::close_button::CloseButtonLayer;
use super::multiple_layer::MultipleLayer;

#[derive(Delegate)]
#[to(Transformable2D, LayerUpdatable)]
pub enum Layer {
    Cursor(CursorLayer),
    Text(TextLayer),
    Shape(ShapeLayer),
    Window(WindowLayer),
    CloseButton(CloseButtonLayer),
    Multiple(MultipleLayer),
    Count(CountLayer),
    Toolbar(ToolbarLayer),
    TextBox(TextBoxLayer),
}


impl Display for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Shape(_) => f.write_str("Shape"),
            Self::Text(_) => f.write_str("Text"),
            Self::Cursor(_) => f.write_str("Cursor"),
            Self::CloseButton(_) => f.write_str("CloseButton"),
            Self::Count(_) => f.write_str("Count"),
            Self::Multiple(_) => f.write_str("Multiple"),
            Self::Window(_) => f.write_str("Window"),
            _ => Ok(()),
        }
    }
}

impl Layer {
    #[inline]
    pub fn require_cursor(&mut self) -> KernelResult<&mut CursorLayer> {
        match self {
            Self::Cursor(cursor) => Ok(cursor),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    #[inline]
    pub fn require_text(&mut self) -> KernelResult<&mut TextLayer> {
        match self {
            Self::Text(text) => Ok(text),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    #[inline]
    pub fn require_text_box(&mut self) -> KernelResult<&mut TextBoxLayer> {
        match self {
            Self::TextBox(text_box) => Ok(text_box),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    #[inline]
    pub fn require_toolbar(&mut self) -> KernelResult<&mut ToolbarLayer> {
        match self {
            Self::Toolbar(toolbar) => Ok(toolbar),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    #[inline]
    pub fn require_shape(&mut self) -> KernelResult<&mut ShapeLayer> {
        match self {
            Self::Shape(shape) => Ok(shape),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    #[inline]
    pub fn require_window(&mut self) -> KernelResult<&mut WindowLayer> {
        match self {
            Self::Window(window) => Ok(window),
            _ => Err(KernelError::FailedOperateLayer(LayerReason::IllegalLayer)),
        }
    }


    #[inline]
    pub fn require_count(&mut self) -> KernelResult<&mut CountLayer> {
        match self {
            Self::Count(count) => Ok(count),
            _ => Err(kernel_error!(
                "Illegal Layer expected Count but was {}",
                &self
            )),
        }
    }


    #[inline]
    pub fn is_window(&self) -> bool {
        matches!(self, Self::Window(_))
    }


    #[inline]
    pub fn is_active_window(&self) -> bool {
        if let Layer::Window(window) = self {
            window.is_active()
        } else {
            false
        }
    }


    #[inline]
    pub fn into_layer_key(self, key: &str) -> LayerKey {
        LayerKey::new(key, self)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_success_update_window_position() {}
}
