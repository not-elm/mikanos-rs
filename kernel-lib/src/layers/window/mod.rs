use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub mod mouse_cursor_window;

pub trait Window {
    /// 現在のウィンドウの状態を描画します。
    ///
    fn draw(&mut self, writer: &mut dyn PixelWritable) -> KernelResult;
}