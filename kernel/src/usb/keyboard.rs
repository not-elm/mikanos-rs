use alloc::string::ToString;
use core::fmt::Write;
use uefi::proto::console::text::Key;

use kernel_lib::layers::terminal::TerminalLayer;
use kernel_lib::layers::LAYERS;
use kernel_lib::serial_println;
use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;
use pci::class_driver::keyboard::Keycode;

use crate::layers::TERMINAL_LAYER_KEY;

pub fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: Keycode) {
    LAYERS
        .lock()
        .update_layer(TERMINAL_LAYER_KEY, |layer| {
            let terminal = layer
                .require_terminal()
                .unwrap();
            match keycode {
                Keycode::ArrowDown => terminal
                    .history_down()
                    .unwrap(),

                Keycode::ArrowUp => terminal.history_up().unwrap(),

                Keycode::Ascii(key) => input_key(key, terminal),
            }
        })
        .unwrap();
}


fn input_key(key: char, terminal: &mut TerminalLayer) {
    match key {
        '\x7F' => {
            terminal.delete_last();
        }
        _ => {
            terminal
                .write_str(key.to_string().as_str())
                .unwrap();
        }
    }
}
