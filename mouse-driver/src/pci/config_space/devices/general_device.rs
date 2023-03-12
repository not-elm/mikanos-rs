use crate::pci::config_space::access::ConfigurationSpace;

/// Header Type 0x0のデバイスを表します。
#[derive(Debug, Clone)]
pub struct GeneralDevice(ConfigurationSpace);

impl GeneralDevice {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }
}
