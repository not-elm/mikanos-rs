use alloc::string::ToString;
use core::fmt::Write;

use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;

use crate::interrupt::timer::TASK_MANAGER;
use crate::layers::{KEYBOARD_TEXT, LAYERS};

pub fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: char) {
    LAYERS
        .lock()
        .update_layer(KEYBOARD_TEXT, |layer| {
            layer
                .require_text()
                .unwrap()
                .write_str(keycode.to_string().as_str())
                .unwrap();
        })
        .unwrap();

    unsafe { operate_count_task_if_need(keycode) };
}


unsafe fn operate_count_task_if_need(keycode: char) {
    match keycode {
        's' => TASK_MANAGER
            .sleep_at(1)
            .unwrap(),

        'w' => TASK_MANAGER
            .wakeup_at(1)
            .unwrap(),

        _ => {}
    }
}
