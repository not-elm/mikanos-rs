use crate::pci::config_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::config_space::io::ConfigurationSpace;

/// Header Type 0x0のデバイスを表します。
#[derive(Debug, Clone)]
pub struct GeneralDevice(ConfigurationSpace);

impl GeneralDevice {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }
}

impl CommonHeaderHoldable for GeneralDevice {
    fn config_space(&self) -> &ConfigurationSpace {
        &self.0
    }
}
