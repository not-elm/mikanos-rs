use core::any::Any;

use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_writable::PixelWritable;

pub mod shape;

pub mod cursor;


pub trait WindowDrawable: Any {
    /// 現在のウィンドウの状態を描画します。
    fn draw(&mut self, transform: &Transform2D, writer: &mut dyn PixelWritable) -> KernelResult;


    /// 自身をAnyにアップキャストします。
    fn any_mut(&mut self) -> &mut dyn Any;
}
