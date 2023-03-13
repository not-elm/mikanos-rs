use core::ops::Deref;

/// コンフィグアドレスレジスタに書き込むためのデータ

pub struct ConfigAddrRegister {
    register_offset: u32,
    function: u32,
    device_slot: u32,
    bus: u32,
}

impl ConfigAddrRegister {
    pub fn new(register_offset: u8, function: u8, device_slot: u8, bus: u8) -> Self {
        Self {
            register_offset: register_offset as u32,
            function: function as u32,
            device_slot: device_slot as u32,
            bus: bus as u32,
        }
    }

    pub fn bus(&self) -> u32 {
        self.bus
    }
    pub fn device_slot(&self) -> u32 {
        self.device_slot
    }
    pub fn function(&self) -> u32 {
        self.function
    }
    pub fn register_offset(&self) -> u32 {
        self.register_offset
    }
    pub fn register_offset_with_mask(&self) -> u32 {
        self.register_offset() & 0xFC
    }

    pub fn to_addr(&self) -> u32 {
        let shift = |d: u32, shift_size: usize| (d << shift_size);

        shift(1, 31)
            | shift(self.bus(), 16)
            | shift(self.device_slot(), 11)
            | shift(self.function(), 8)
            | self.register_offset_with_mask()
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
