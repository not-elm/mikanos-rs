use alloc::string::ToString;
use core::fmt::Write;

use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;

use crate::layers::{KEYBOARD_TEXT, LAYERS};
use crate::task::TASK_MANAGER;

pub fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: char) {
    LAYERS
        .layers_mut()
        .lock()
        .borrow_mut()
        .update_layer(KEYBOARD_TEXT, |layer| {
            layer
                .require_text()
                .unwrap()
                .write_str(keycode.to_string().as_str())
                .unwrap();
        })
        .unwrap();

    unsafe {
        // if keycode == 's' {
        //     TASK_MANAGER
        //         .sleep_at(1)
        //         .unwrap();
        // } else if keycode == 'w' {
        //     TASK_MANAGER
        //         .wakeup_at(1)
        //         .unwrap();
        // }
    }
}