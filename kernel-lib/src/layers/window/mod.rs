use alloc::boxed::Box;

use common_lib::math::vector::Vector2D;

use crate::layers::window::drawers::WindowDrawable;
use common_lib::transform::Transform2D;

pub mod drawers;


pub struct Window<Draw> {
    drawer: Draw,
    transform: Transform2D,
}


impl<Draw> Window<Draw> {
    pub const fn new(drawer: Draw, transform: Transform2D) -> Window<Draw> {
        Self { drawer, transform }
    }


    pub fn move_window(&mut self, pos: Vector2D<usize>) {
        self.transform.set_pos(pos);
    }


    pub fn drawer(&mut self) -> &mut Draw {
        &mut self.drawer
    }


    pub fn set_transform(&mut self, transform: Transform2D) {
        self.transform = transform;
    }


    pub fn transform_ref(&self) -> &Transform2D {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
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
        Window::new(Box::new(self.drawer), self.transform)
    }
}
