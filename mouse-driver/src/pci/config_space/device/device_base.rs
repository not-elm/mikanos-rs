use crate::pci::config_space::access::ConfigurationSpace;
use crate::pci::config_space::common_header::common_header_holdable::CommonHeaderHoldable;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DeviceBase(ConfigurationSpace);

impl DeviceBase {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }
}

impl CommonHeaderHoldable for DeviceBase {
    fn config_space(&self) -> &ConfigurationSpace {
        &self.0
    }
}
