use core::any::Any;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::status::WindowStatus;

pub mod background;
pub mod mouse_cursor;


pub trait WindowDrawable: Any {
    /// 現在のウィンドウの状態を描画します。
    fn draw(&mut self, status: &WindowStatus, writer: &mut dyn PixelWritable) -> KernelResult;


    /// 自身をAnyにアップキャストします。
    fn any_mut(&mut self) -> &mut dyn Any;
}
