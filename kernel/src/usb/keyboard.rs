use alloc::string::ToString;
use core::fmt::Write;

use kernel_lib::layers::multiple_layer::LayerFindable;
use kernel_lib::layers::terminal::TerminalLayer;
use kernel_lib::layers::window::WindowLayer;
use kernel_lib::layers::LAYERS;
use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;
use pci::class_driver::keyboard::Keycode;

use crate::layers::KEYBOARD_TEXT;

pub fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: Keycode) {
    LAYERS
        .lock()
        .update_active_layer(|layer| {
            if let Ok(terminal) = layer.require_terminal() {
                return input_terminal(keycode, terminal);
            }

            if let Ok(window) = layer.require_window() {
                keyboard_text_box(keycode, window)
            }
        })
        .unwrap();
}


fn keyboard_text_box(keycode: Keycode, window: &mut WindowLayer) {
    if let Keycode::Ascii(c) = keycode {
        if let Some(text_box) = window.find_by_key_mut(KEYBOARD_TEXT) {
            let text_box = text_box
                .require_text_box()
                .unwrap();
            match c {
                '\x7F' => {
                    text_box.delete_last();
                }
                c => {
                    text_box
                        .write_str(c.to_string().as_str())
                        .unwrap();
                }
            }
        }
    }
}


fn input_terminal(keycode: Keycode, terminal: &mut TerminalLayer) {
    match keycode {
        Keycode::ArrowDown => terminal
            .history_down()
            .unwrap(),

        Keycode::ArrowUp => terminal.history_up().unwrap(),

        Keycode::Ascii(key) => input_key(key, terminal),
    }
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
