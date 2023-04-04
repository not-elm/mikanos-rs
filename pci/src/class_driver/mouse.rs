use common_lib::vector::Vector2D;

mod mouse_default_driver;
pub mod mouse_driver_factory;
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
