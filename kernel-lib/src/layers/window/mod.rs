use alloc::boxed::Box;
use core::any::Any;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;

use crate::error::KernelResult;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::window::status::WindowStatus;

pub mod drawers;
pub mod status;

pub trait WindowDrawable: Any {
    /// 現在のウィンドウの状態を描画します。
    fn draw(&mut self, pos: Vector2D<usize>, writer: &mut dyn PixelWritable) -> KernelResult;
    fn any_mut(&mut self) -> &mut dyn Any;
}


pub struct Window<Draw> {
    drawer: Draw,
    status: WindowStatus,
}


impl<Draw> Window<Draw> {
    pub const fn new(
        drawer: Draw,
        background_color: PixelColor,
        color: PixelColor,
        pos: Vector2D<usize>,
        size: Size,
    ) -> Window<Draw> {
        Self {
            drawer,
            background_color,
            color,
            size,
            pos,
        }
    }


    pub fn move_window(&mut self, pos: Vector2D<usize>) {
        self.pos = pos;
    }


    pub fn drawer(&mut self) -> &mut Draw {
        &mut self.drawer
    }


    pub fn pos(&self) -> Vector2D<usize> {
        self.pos
    }
}


impl<'draw, Draw> Window<Draw>
where
    Draw: WindowDrawable + 'draw,
{
    pub fn drawer_down_cast_mut<D>(&'draw mut self) -> Option<&mut D>
    where
        D: WindowDrawable + 'draw,
    {
        self.drawer
            .any_mut()
            .downcast_mut()
    }


    pub fn into_dyn(self) -> Window<Box<dyn WindowDrawable>> {
        Window::new(
            Box::new(self.drawer),
            self.background_color,
            self.color,
            self.pos,
            self.size,
        )
    }
}
