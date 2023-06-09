use alloc::vec::Vec;

use crate::class_driver::ClassDriverOperate;
use crate::class_driver::keyboard::keycode::Keycode;
use crate::class_driver::keyboard::subscribe::{KeyboardSubscribable, KeyModifier};
use crate::error::PciResult;

pub struct KeyboardDriver<F> {
    data_buff: [u8; 8],
    auto_upper: bool,
    modifiers: Vec<KeyModifier>,
    keycodes: Vec<char>,
    subscribe: F,
}


impl<F> KeyboardDriver<F>
    where
        F: KeyboardSubscribable,
{
    pub(crate) fn new(auto_upper: bool, subscribe: F) -> KeyboardDriver<F> {
        Self {
            data_buff: [0; 8],
            auto_upper,
            modifiers: Vec::with_capacity(8),
            keycodes: Vec::with_capacity(8),
            subscribe,
        }
    }


    fn keycodes(&self) -> Vec<char> {
        self.data_buff[2..]
            .iter()
            .filter_map(|b| {
                if self.auto_upper {
                    Keycode::new(*b).upper_char()
                } else {
                    Keycode::new(*b).char()
                }
            })
            .collect::<Vec<char>>()
    }
}


impl<F> ClassDriverOperate for KeyboardDriver<F>
    where
        F: KeyboardSubscribable,
{
    fn on_data_received(&mut self) -> PciResult {
        let prev_modifiers = self.modifiers.clone();

        let prev_keys = self.keycodes.clone();
        self.keycodes = self.keycodes();

        self.subscribe.subscribe(
            prev_modifiers.as_slice(),
            self.modifiers.as_slice(),
            prev_keys.as_slice(),
            self.keycodes.as_slice(),
        );
        Ok(())
    }


    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }


    fn data_buff_len(&self) -> u32 {
        8
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::class_driver::keyboard::builder::Builder;

    #[test]
    fn it_keycodes() {
        let mut keyboard = Builder::new()
            .mock();

        keyboard.data_buff = [0, 0, 0x04, 0x2F, 0, 0, 0, 0];
        assert_eq!(keyboard.keycodes().len(), 2);
        assert_eq!(keyboard.keycodes(), vec!['a', '{']);
    }


    #[test]
    fn it_keycodes_upper_case() {
        let mut keyboard = Builder::new()
            .auto_upper_if_shift()
            .mock();

        keyboard.data_buff = [
            0, 0, 0x04, 0x2F, 0x05, 0, 0x06, 0,
        ];
        assert_eq!(keyboard.keycodes().len(), 4);
        assert_eq!(keyboard.keycodes(), vec!['A', '{', 'B', 'C']);
    }
}
