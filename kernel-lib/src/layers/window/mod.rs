use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::mouse_cursor_window::MouseCursorWindow;

pub mod mouse_cursor_window;

pub trait WindowBuffer {
    /// 現在のウィンドウの状態を描画します。
    ///
    fn draw(&mut self, writer: &mut dyn PixelWritable) -> KernelResult;
}

#[derive(Debug)]
pub enum Window {
    Console,
    Mouse(MouseCursorWindow),
}


impl Window {
    pub fn new_mouse_cursor(scale: Vector2D<usize>) -> Self {
        Self::Mouse(MouseCursorWindow::new(scale))
    }
}


