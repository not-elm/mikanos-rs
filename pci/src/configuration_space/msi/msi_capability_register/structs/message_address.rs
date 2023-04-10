/// この構造体はMessageAddressを保有します。
///
/// 64bitの場合、UpperAddressが上位32Bitsとなり、合計64bitsになります。
///
/// [Document](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/msi-registers.html)
#[derive(Debug, Clone)]
pub struct MessageAddress {
    message_lower_addr: u32,
    message_upper_addr: u32,
}

impl MessageAddress {
    pub const fn new(message_lower_addr: u32, message_upper_addr: u32) -> Self {
        Self {
            message_lower_addr,
            message_upper_addr,
        }
    }

    pub fn set_message_addr(&mut self, message_addr: usize) {
        self.message_lower_addr = (message_addr & 0xFFFF_FFFF) as u32;
        if (u32::MAX as usize) < message_addr {
            self.message_upper_addr = ((message_addr >> 32) & 0xFFFF_FFFF) as u32
        }
    }

    pub fn message_addr(&self, is_64bit_addr: bool) -> usize {
        if is_64bit_addr {
            ((self.message_upper_addr as usize) << 32) | self.message_lower_addr as usize
        } else {
            self.message_lower_addr as usize
        }
    }


    pub fn message_lower_addr(&self) -> u32 {
        self.message_lower_addr
    }


    pub fn message_upper_addr(&self) -> u32 {
        self.message_upper_addr
    }
}


#[cfg(test)]
mod tests {
    use crate::configuration_space::msi::msi_capability_register::structs::message_address::MessageAddress;

    #[test]
    fn it_when_32_bits_read_take_only_lower_addr() {
        let addr = msi_message_address_register().message_addr(false);
        assert_eq!(addr, 0x04);
    }


    #[test]
    fn it_when_64_bits_read_take_only_lower_addr() {
        let addr = msi_message_address_register().message_addr(true);
        assert_eq!(addr, 0x04 << 32 | 0x04);
    }


    fn msi_message_address_register() -> MessageAddress {
        MessageAddress::new(0x04, 0x04)
    }
}
