use kernel_lib::println;

use crate::pci::config_space::access::ConfigurationSpace;
use crate::pci::config_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::config_space::device::PciDevice;

#[derive(Debug)]
pub struct MultipleFunctionDevice {
    config_space: ConfigurationSpace,
    function: u8,
}

impl MultipleFunctionDevice {
    pub(crate) fn new(config_space: ConfigurationSpace) -> Self {
        Self {
            config_space,
            function: 1,
        }
    }
}

impl CommonHeaderHoldable for MultipleFunctionDevice {
    fn config_space(&self) -> &ConfigurationSpace {
        &self.config_space
    }
}

impl Iterator for MultipleFunctionDevice {
    type Item = PciDevice;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_sequence()
    }
}

impl MultipleFunctionDevice {
    fn next_sequence(&mut self) -> Option<PciDevice> {
        println!("start {:?}", self.function);
        const MAX_FUNCTION_SIZE: u8 = 8;
        if MAX_FUNCTION_SIZE <= self.function {
            return None;
        }

        let next_config_space = ConfigurationSpace::try_new(
            self.config_space.bus(),
            self.config_space.device_slot(),
            self.function,
        );
        println!("next{:?}", next_config_space);
        println!("end {:?}", self.function);
        self.function += 1;
        if let Some(next_config_space) = next_config_space.and_then(|c| c.cast_device()) {
            return Some(next_config_space);
        }

        return self.next_sequence();
    }
}
