use crate::pci::config_space::access::config_address_register::ConfigAddrRegister;
use crate::pci::config_space::access::intel_x86_io::{fetch_config_data, write_config_addr};
use crate::pci::config_space::devices::common_header_loadable::exists_device;

pub mod config_address_register;
pub mod intel_x86_io;

#[derive(Clone, Debug)]
pub struct ConfigurationSpace {
    bus: u8,
    device_slot: u8,
    function: u8,
}

impl ConfigurationSpace {
    pub fn try_new(bus: u8, device_slot: u8, function: u8) -> Option<Self> {
        if exists_device(0) {
            return Some(ConfigurationSpace::new(bus, device_slot, function));
        } else {
            None
        }
    }
    pub fn fetch_data_offset_at(&self, offset: u8) -> u32 {
        write_config_addr(self.config_addr_at(offset));
        fetch_config_data()
    }
    fn new(bus: u8, device_slot: u8, function: u8) -> Self {
        Self {
            bus,
            device_slot,
            function,
        }
    }
    fn config_addr_at(&self, offset: u8) -> ConfigAddrRegister {
        ConfigAddrRegister::new()
            .with_enabled(true)
            .with_bus(self.bus)
            .with_device_slot(self.device_slot)
            .with_function(self.function)
            .with_register_offset(offset)
    }
}

#[cfg(test)]
mod tests {
    use crate::pci::config_space::access::ConfigurationSpace;

    #[test]
    fn it_new_first_offset() {
        let p = ConfigurationSpace::new(1, 2, 3).config_addr_at(0);
        let inner = *p;
        assert_eq!(p.enabled(), true);
        assert_eq!((inner >> 31), 1);

        assert_eq!(p.bus(), 1);
        assert_eq!(((inner >> 16) & 0xFF), 1);

        assert_eq!(p.device_slot(), 2);
        assert_eq!(((inner >> 11) & 0b11111), 2);

        assert_eq!(p.function(), 3);
        assert_eq!(((inner >> 8) & 0b111), 3);

        assert_eq!(p.register_offset(), 0);
        assert_eq!((inner & 0xFC), 0);
    }
}
