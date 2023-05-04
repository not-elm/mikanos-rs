use alloc::boxed::Box;

use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::console::ConsoleLayer;
use crate::layers::cursor::CursorLayer;
use crate::layers::layer_key::LayerKey;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::shape::ShapeLayer;

pub enum Layer {
    Cursor(CursorLayer),
    Console(Box<ConsoleLayer>),
    Shape(ShapeLayer),
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


    pub fn into_layer_key(self, key: &str) -> LayerKey {
        LayerKey::new(key, self)
    }
}


impl Transformable2D for Layer {
    fn move_to(&mut self, pos: Vector2D<usize>) {
        match self {
            Self::Cursor(cursor) => cursor.move_to(pos),
            Self::Shape(shape) => shape.move_to(pos),
            _ => {}
        }
    }


    fn resize(&mut self, size: Size) {
        match self {
            Self::Console(console) => console.resize(size),
            Self::Cursor(cursor) => cursor.resize(size),
            Self::Shape(shape) => shape.resize(size),
        }
    }


    fn rect(&self) -> Rectangle<usize> {
        match self {
            Self::Console(console) => console.rect(),
            Self::Cursor(cursor) => cursor.rect(),
            Self::Shape(shape) => shape.rect(),
        }
    }


    fn transform_ref(&self) -> &Transform2D {
        match self {
            Self::Console(console) => console.transform_ref(),
            Self::Shape(shape) => shape.transform_ref(),
            Self::Cursor(cursor) => cursor.transform_ref(),
        }
    }
}


impl LayerUpdatable for Layer {
    fn update_shadow_buffer(
        &mut self,
        shadow_buffer: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        match self {
            Self::Cursor(cursor) => cursor.update_shadow_buffer(shadow_buffer, draw_area),
            Self::Console(console) => console.update_shadow_buffer(shadow_buffer, draw_area),
            Self::Shape(shape) => shape.update_shadow_buffer(shadow_buffer, draw_area),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_success_update_window_position() {}
}
