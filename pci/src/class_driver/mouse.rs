use common_lib::vector::Vector2D;
use core::cmp::max;

mod mouse_default_driver;
pub mod mouse_driver_factory;
pub mod mouse_subscribable;
pub mod mouse_subscribe_driver;

const MOUSE_DATA_BUFF_SIZE: usize = 3;
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
pub(crate) fn cursor_pos(buff: &[i8]) -> Vector2D<isize> {
    Vector2D::new(buff[1] as isize, buff[2] as isize)
}
pub(crate) fn current_cursor_pos(prev_pos: Vector2D<usize>, data_buff: &[i8]) -> Vector2D<usize> {
    let relative = cursor_pos(data_buff);
    Vector2D::new(
        max(prev_pos.x() as isize + relative.x(), 0) as usize,
        max(prev_pos.y() as isize + relative.y(), 0) as usize,
    )
}
