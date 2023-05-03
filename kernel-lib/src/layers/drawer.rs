use core::any::Any;

use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::pixel::writer::enum_pixel_writer::EnumPixelWriter;
use crate::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};

pub mod shape_drawer;

pub mod console_drawer;
pub mod cursor;
pub mod rect_colors;


pub trait LayerDrawable: Any {
    /// 指定された領域内で現在のウィンドウの状態を描画します。
    fn draw_in_area(
        &mut self,
        pixels: &mut [u8],
        pixel_writer: &mut EnumPixelWriter,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult;


    /// 自身をAnyにアップキャストします。
    fn any_mut(&mut self) -> &mut dyn Any;
}
