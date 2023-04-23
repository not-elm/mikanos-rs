use common_lib::math::rectangle::Rectangle;
use core::any::Any;

use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::pixel_writable::PixelWritable;

pub mod shape;

pub mod cursor;


pub trait WindowDrawable: Any {
    /// 指定された領域内で現在のウィンドウの状態を描画します。
    fn draw_in_area(
        &mut self,
        window_transform: &Transform2D,
        draw_rect: &Rectangle<usize>,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult;


    /// 現在のウィンドウの状態を描画します。
    fn draw(
        &mut self,
        window_transform: &Transform2D,
        writer: &mut dyn PixelWritable,
    ) -> KernelResult {
        self.draw_in_area(window_transform, &window_transform.rect(), writer)
    }

    /// 自身をAnyにアップキャストします。
    fn any_mut(&mut self) -> &mut dyn Any;
}
