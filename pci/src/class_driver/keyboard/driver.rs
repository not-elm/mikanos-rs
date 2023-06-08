use alloc::vec::Vec;

use crate::class_driver::ClassDriverOperate;
use crate::class_driver::keyboard::subscribe::KeyboardSubscribable;
use crate::error::PciResult;

pub struct KeyboardDriver<F> {
    data_buff: [u8; 8],
    auto_upper: bool,
    subscribe: F,
}


impl<F> KeyboardDriver<F> where F: KeyboardSubscribable {
    pub(crate) const fn new(auto_upper: bool, subscribe: F) -> KeyboardDriver<F> {
        Self {
            data_buff: [0; 8],
            auto_upper,
            subscribe,
        }
    }

    fn keycodes(&self) -> Vec<char> {
        self.data_buff[2..]
            .iter()
            .map(|b| char::from(*b))
            .filter(|c| *c != '\0')
            .collect::<Vec<char>>()
    }
}


impl<F> ClassDriverOperate for KeyboardDriver<F>
    where
        F: KeyboardSubscribable,
{
    fn on_data_received(&mut self) -> PciResult {
        let keys = self.keycodes();

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
    use crate::class_driver::keyboard::subscribe::KeyModifier;

    #[test]
    fn it_keycodes() {
        let mut keyboard = Builder::new()
            .build(|a: &[KeyModifier], _: &[KeyModifier]| {});

        keyboard.data_buff = [0, 0, 0x04, 0x3E, 0, 0, 0, 0];
        assert_eq!(keyboard.keycodes().len(), 2);
        assert_eq!(keyboard.keycodes(), vec!['a', 'a']);
    }
}
