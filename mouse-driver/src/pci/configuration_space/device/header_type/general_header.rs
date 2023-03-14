use crate::pci::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::configuration_space::ConfigurationSpace;

/// Header Type 0x0のデバイスを表します。
#[derive(Debug, Clone)]
pub struct GeneralHeader(ConfigurationSpace);

impl GeneralHeader {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }
}

impl CommonHeaderHoldable for GeneralHeader {
    fn as_config_space(&self) -> &ConfigurationSpace {
        &self.0
    }
}
