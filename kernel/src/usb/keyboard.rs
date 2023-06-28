use alloc::string::ToString;
use core::fmt::Write;

use kernel_lib::layers::LAYERS;
use kernel_lib::task::TASK_MANAGER;
use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;

use crate::layers::KEYBOARD_TEXT;

pub fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: char) {
    update_text_box_keys(keycode);

    unsafe { operate_count_task_if_need(keycode) };
}


fn update_text_box_keys(keycode: char) {
    LAYERS
        .lock()
        .update_layer(KEYBOARD_TEXT, |layer| {
            let text_layer = layer
                .require_text_box()
                .unwrap();
            match keycode {
                '\x7F' => {
                    text_layer.delete_last();
                }
                _ => {
                    text_layer
                        .write_str(keycode.to_string().as_str())
                        .unwrap();
                }
            }
        })
        .unwrap();
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
