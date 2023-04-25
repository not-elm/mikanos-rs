use core::any::Any;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;

pub mod shape;

pub mod cursor;
pub mod rect_colors;


pub trait WindowDrawable: Any {
    /// 指定された領域内で現在のウィンドウの状態を描画します。
    fn draw_in_area(
        &mut self,
        window_transform: &Transform2D,
        draw_rect: &Rectangle<usize>,
        writer: &mut dyn PixelFlushable,
    ) -> KernelResult;


    /// 現在のウィンドウの状態を描画します。
    fn draw(
        &mut self,
        window_transform: &Transform2D,
        writer: &mut dyn PixelFlushable,
    ) -> KernelResult {
        self.draw_in_area(window_transform, &window_transform.rect(), writer)
    }

    /// 自身をAnyにアップキャストします。
    fn any_mut(&mut self) -> &mut dyn Any;
}
