use alloc::boxed::Box;

use common_lib::math::vector::Vector2D;
use common_lib::transform::Transform2D;

use crate::error::LayerReason::InvalidCastWindowDrawer;
use crate::error::{KernelError, KernelResult};
use crate::layers::drawer::LayerDrawable;

// pub struct Window<Draw> {
//     drawer: Draw,
//     transform: Transform2D,
// }
//
//
// impl<Draw> Window<Draw> {
//     pub const fn new(drawer: Draw, transform: Transform2D) -> Window<Draw> {
//         Self { drawer, transform }
//     }
//
//
//     pub fn move_window(&mut self, pos: Vector2D<usize>) {
//         self.transform.set_pos(pos);
//     }
//
//
//     pub fn drawer(&mut self) -> &mut Draw {
//         &mut self.drawer
//     }
//
//
//     pub fn set_transform(&mut self, transform: Transform2D) {
//         self.transform = transform;
//     }
//
//
//     pub fn transform_ref(&self) -> &Transform2D {
//         &self.transform
//     }
//
//
//     pub fn transform_mut(&mut self) -> &mut Transform2D {
//         &mut self.transform
//     }
// }
//
//
// impl Window<Box<dyn LayerDrawable>> {
//     pub fn drawer_down_cast_mut<Draw>(&mut self) -> KernelResult<&mut Draw>
//     where
//         Draw: LayerDrawable,
//     {
//         self.drawer
//             .any_mut()
//             .downcast_mut()
//             .ok_or(KernelError::FailedOperateLayer(InvalidCastWindowDrawer))
//     }
// }
//
//
// impl<'draw, Draw> Window<Draw>
// where
//     Draw: LayerDrawable + 'draw,
// {
//     pub fn drawer_down_cast_mut<D>(&'draw mut self) -> Option<&mut D>
//     where
//         D: LayerDrawable + 'draw,
//     {
//         self.drawer
//             .any_mut()
//             .downcast_mut()
//     }
//
//
//     pub fn into_dyn(self) -> Window<Box<dyn LayerDrawable>> {
//         Window::new(Box::new(self.drawer), self.transform)
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use alloc::boxed::Box;
//
//     use common_lib::math::size::Size;
//     use common_lib::math::vector::Vector2D;
//     use common_lib::transform::Transform2D;
//
//     use crate::layers::drawer::cursor::cursor_drawer::CursorDrawer;
//     use crate::layers::drawer::LayerDrawable;
//     use crate::layers::window::Window;
//
//     #[test]
//     fn it_down_cast_to_mouse_cursor_drawer() {
//         let mut window: Window<Box<dyn LayerDrawable>> = Window::new(
//             Box::<CursorDrawer>::default(),
//             Transform2D::new(Vector2D::zeros(), Size::new(100, 100)),
//         );
//         assert!(window
//             .drawer_down_cast_mut::<CursorDrawer>()
//             .is_ok());
//     }
// }
