use alloc::boxed::Box;

use common_lib::math::vector::Vector2D;

use crate::layers::window::drawers::WindowDrawable;
use crate::layers::window::status::WindowStatus;

pub mod drawers;
pub mod status;


pub struct Window<Draw> {
    drawer: Draw,
    status: WindowStatus,
}


impl<Draw> Window<Draw> {
    pub const fn new(drawer: Draw, status: WindowStatus) -> Window<Draw> {
        Self { drawer, status }
    }


    pub fn move_window(&mut self, pos: Vector2D<usize>) {
        self.status.set_pos(pos);
    }


    pub fn drawer(&mut self) -> &mut Draw {
        &mut self.drawer
    }


    pub fn set_window_status(&mut self, window_status: WindowStatus) {
        self.status = window_status;
    }


    pub fn status_ref(&self) -> &WindowStatus {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut WindowStatus {
        &mut self.status
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
        Window::new(Box::new(self.drawer), self.status)
    }
}
