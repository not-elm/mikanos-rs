use core::ops::Deref;

use bitfield_struct::bitfield;

/// コンフィグアドレスレジスタに書き込むためのデータ
#[bitfield(u32)]
pub struct ConfigAddrRegister {
    #[bits(2)]
    _preserve1: usize,
    #[bits(6)]
    pub register_offset: u8,
    #[bits(3)]
    pub function: u8,
    #[bits(5)]
    pub device_slot: u8,
    pub bus: u8,
    #[bits(7)]
    _preserve2: u8,
    pub enabled: bool,
}

impl ConfigAddrRegister {
    pub(crate) fn new_first_offset(bus: u8, device_slot: u8, function: u8) -> Self {
        ConfigAddrRegister::new()
            .with_enabled(true)
            .with_bus(bus)
            .with_device_slot(device_slot)
            .with_function(function)
            .with_register_offset(0)
    }
}

impl Deref for ConfigAddrRegister {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::pci::config_space::access::config_address_register::ConfigAddrRegister;

    #[test]
    fn it_new_default() {
        let p = ConfigAddrRegister::new();
        assert_eq!(p.register_offset(), 0);
        assert_eq!(0, p.0);
    }

    #[test]
    fn it_deref() {
        let p = ConfigAddrRegister::new().with_register_offset(1);
        assert_eq!((*p >> 2), 1);
    }
}
