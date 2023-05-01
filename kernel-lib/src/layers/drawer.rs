use core::any::Any;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};

pub mod shape_drawer;

pub mod cursor;
pub mod rect_colors;


pub trait LayerDrawable: Any {
    /// 指定された領域内で現在のウィンドウの状態を描画します。
    fn draw_in_area(
        &mut self,
        transform: &Transform2D,
        pixels: &mut [PixelColor],
        draw_rect: &Rectangle<usize>,
    ) -> KernelResult;


    /// 現在のウィンドウの状態を描画します。
    fn draw(&mut self, transform: &Transform2D, pixels: &mut [PixelColor]) -> KernelResult {
        self.draw_in_area(transform, pixels, &transform.rect())
    }


    /// 自身をAnyにアップキャストします。
    fn any_mut(&mut self) -> &mut dyn Any;
}
