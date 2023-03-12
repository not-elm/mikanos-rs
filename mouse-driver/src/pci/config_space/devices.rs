use crate::pci::config_space::devices::general_device::GeneralDevice;

pub mod class_code;
pub mod common_header_loadable;
pub mod general_device;
pub mod sub_class;

pub enum PciDevice {
    General(GeneralDevice),
}
