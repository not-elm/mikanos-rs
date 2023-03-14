use crate::pci::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::configuration_space::device::function::Function;
use crate::pci::configuration_space::ConfigurationSpace;

#[derive(Debug)]
pub struct MultipleFunctionDevice {
    config_space: ConfigurationSpace,
    function: u8,
}

impl MultipleFunctionDevice {
    pub(crate) fn new(config_space: ConfigurationSpace) -> Self {
        let function = config_space.function() + 1;
        Self {
            config_space,
            function,
        }
    }
}

impl CommonHeaderHoldable for MultipleFunctionDevice {
    fn as_config_space(&self) -> &ConfigurationSpace {
        &self.config_space
    }
}

impl Iterator for MultipleFunctionDevice {
    type Item = Function;

    fn next(&mut self) -> Option<Self::Item> {
        const MAX_FUNCTION_SIZE: u8 = 8;
        if MAX_FUNCTION_SIZE <= self.function {
            return None;
        }

        let next_config_space = ConfigurationSpace::try_new(
            self.config_space.bus(),
            self.config_space.device_slot(),
            self.function,
        );

        self.function += 1;
        if let Some(next_config_space) = next_config_space.map(|c| c.cast_device()) {
            return Some(next_config_space);
        }

        self.next()
    }
}
