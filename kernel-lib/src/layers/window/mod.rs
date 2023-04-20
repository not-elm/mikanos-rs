mod mouse_cursor_window;

use crate::gop::pixel::pixel_writable::PixelWritable;

pub trait Window {
    /// 現在のウィンドウの状態を描画します。
    ///
    fn draw(&mut self, writer: &mut dyn PixelWritable);
}