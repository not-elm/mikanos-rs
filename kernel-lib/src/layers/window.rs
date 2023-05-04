// pub struct Window<Draw> {
//     layer_updatable: Draw,
//     transform: Transform2D,
// }
//
//
// impl<Draw> Window<Draw> {
//     pub const fn new(layer_updatable: Draw, transform: Transform2D) -> Window<Draw> {
//         Self { layer_updatable, transform }
//     }
//
//
//     pub fn move_window(&mut self, pos: Vector2D<usize>) {
//         self.transform.set_pos(pos);
//     }
//
//
//     pub fn layer_updatable(&mut self) -> &mut Draw {
//         &mut self.layer_updatable
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
//         self.layer_updatable
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
//         self.layer_updatable
//             .any_mut()
//             .downcast_mut()
//     }
//
//
//     pub fn into_dyn(self) -> Window<Box<dyn LayerDrawable>> {
//         Window::new(Box::new(self.layer_updatable), self.transform)
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
//     use crate::layers::layer_updatable::cursor::cursor_drawer::CursorDrawer;
//     use crate::layers::layer_updatable::LayerDrawable;
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
