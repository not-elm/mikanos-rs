use crate::pci::config_space::access::config_address_register::ConfigAddrRegister;
use crate::pci::config_space::access::intel_x86_io::{fetch_config_data, write_config_addr};
use crate::pci::config_space::devices::common_header_loadable::exists_device;
use crate::pci::config_space::devices::general_device::GeneralDevice;

pub mod common_header_loadable;
pub mod general_device;

pub enum PciDevice {
    General(GeneralDevice),
}

pub fn fetch_pci_device(bus: u8, device_slot: u8, function: u8) -> Option<PciDevice> {
    let config_addr_register = ConfigAddrRegister::new()
        .with_bus(bus)
        .with_device_slot(device_slot)
        .with_function(function)
        .with_register_offset(0);

    write_config_addr(config_addr_register);
    if (!exists_device(fetch_config_data())) {
        return None;
    }

    None
}
