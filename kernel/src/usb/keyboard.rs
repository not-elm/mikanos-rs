use alloc::string::ToString;
use core::fmt::Write;

use kernel_lib::layers::LAYERS;
use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;

use crate::layers::TERMINAL_LAYER_KEY;

pub fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: char) {
    update_text_box_keys(keycode);
}


fn update_text_box_keys(keycode: char) {
    LAYERS
        .lock()
        .update_layer(TERMINAL_LAYER_KEY, |layer| {
            let text_layer = layer
                .require_terminal()
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
