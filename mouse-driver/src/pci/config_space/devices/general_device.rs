use crate::pci::config_space::access::config_address_register::ConfigAddrRegister;

/// Header Type 0x0のデバイスを表します。
pub struct GeneralDevice {
    config_address: ConfigAddrRegister,
}

impl GeneralDevice {
    pub fn new(config_address: ConfigAddrRegister) -> Self {
        Self { config_address }
    }
}
