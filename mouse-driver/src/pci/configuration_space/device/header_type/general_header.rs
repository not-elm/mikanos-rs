use crate::pci::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::configuration_space::ConfigurationSpace;

/// Header Type 0x0のデバイスを表します。
#[derive(Debug, Clone)]
pub struct GeneralHeader(ConfigurationSpace);

impl GeneralHeader {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }

    pub fn mmio_base_addr(&self) -> usize {
        let bar0 = self.as_config_space().fetch_data_offset_at(0x10) as usize;
        let bar1 = self.as_config_space().fetch_data_offset_at(0x14) as usize;
        (bar1 << 32) | (bar0 & 0xFF_FF_FF_F0)
    }
}

impl CommonHeaderHoldable for GeneralHeader {
    fn as_config_space(&self) -> &ConfigurationSpace {
        &self.0
    }
}
