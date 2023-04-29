use core::cmp::max;

use common_lib::math::vector::Vector2D;

use crate::class_driver::boot_protocol_buffer::BootProtocolBuffer;
use crate::class_driver::mouse::MouseButton::{Button1, Button2, Button3, DeviceSpecific};

mod mouse_default_driver;
pub mod mouse_driver_factory;
pub mod mouse_subscribable;
pub mod mouse_subscribe_driver;


const MOUSE_DATA_BUFF_SIZE: usize = 3;


#[derive(Debug, Clone)]
pub enum MouseButton {
    Button1,
    Button2,
    Button3,
    DeviceSpecific(i8),
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


/// データバッファから、押下されているボタンを取得します。
///
///
/// [HID] : 71P
///
/// [HID]: https://www.usb.org/sites/default/files/documents/hid1_11.pdf
pub(crate) fn mouse_button_boot_protocol(data_buff: BootProtocolBuffer) -> Option<MouseButton> {
    let button_data = data_buff.buff()[0];

    match button_data {
        0b0000_0000 => None,
        0b0000_0001 => Some(Button1),
        0b0000_0010 => Some(Button2),
        0b0000_0100 => Some(Button3),
        _ => Some(DeviceSpecific(button_data >> 4)),
    }
}
