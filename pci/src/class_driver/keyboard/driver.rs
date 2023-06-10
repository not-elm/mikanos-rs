use crate::class_driver::keyboard::keycode::Keycode;
use crate::class_driver::keyboard::subscribe::{BoxedKeyboardSubscriber, LEFT_SHIFT, RIGHT_SHIFT};
use crate::class_driver::ClassDriverOperate;
use crate::error::PciResult;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct KeyboardDriver {
    prev_buf: [u8; 8],
    data_buff: [u8; 8],
    auto_upper: bool,
    subscribe: BoxedKeyboardSubscriber,
}


impl KeyboardDriver {
    pub(crate) fn new(auto_upper: bool, subscribe: BoxedKeyboardSubscriber) -> KeyboardDriver {
        Self {
            prev_buf: [0; 8],
            data_buff: [0; 8],
            auto_upper,
            subscribe,
        }
    }


    fn keycodes(&self) -> Vec<char> {
        self.data_buff[2..]
            .iter()
            .filter(|key| !self.prev_buf[2..].contains(key))
            .filter_map(|key| self.keycode(*key))
            .collect()
    }


    fn keycode(&self, b: u8) -> Option<char> {
        if self.auto_upper && self.pushing_shift() {
            Keycode::new(b).upper_char()
        } else {
            Keycode::new(b).char()
        }
    }


    fn pushing_shift(&self) -> bool {
        (self.data_buff[0] & (LEFT_SHIFT | RIGHT_SHIFT)) != 0
    }
}


impl ClassDriverOperate for KeyboardDriver {
    fn on_data_received(&mut self) -> PciResult {
        self.keycodes()
            .iter()
            .for_each(|key| {
                self.subscribe
                    .subscribe(self.data_buff[0], *key);
            });

        self.prev_buf
            .copy_from_slice(&self.data_buff);

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
    use crate::class_driver::keyboard::subscribe::{LEFT_SHIFT, RIGHT_SHIFT};

    #[test]
    fn it_keycodes() {
        let mut keyboard = Builder::new().mock();

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
            LEFT_SHIFT | RIGHT_SHIFT,
            0,
            0x04,
            0x2F,
            0x05,
            0,
            0x06,
            0,
        ];
        assert_eq!(keyboard.keycodes().len(), 4);
        assert_eq!(keyboard.keycodes(), vec!['A', '{', 'B', 'C']);
    }
}
