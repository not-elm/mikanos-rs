use crate::pci::config_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::config_space::device_iter::device_slots::DeviceSlots;
use crate::pci::config_space::io::ConfigurationSpace;

#[derive(Debug)]
pub struct PciBrideDevice(ConfigurationSpace);

impl PciBrideDevice {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }
}

impl PciBrideDevice {
    pub fn children(&self) -> DeviceSlots {
        DeviceSlots::new(self.bus_numbers())
    }
    pub fn bus_numbers(&self) -> u8 {
        convert_to_bus_numbers(self.0.fetch_data_offset_at(0x18))
    }
}

fn convert_to_bus_numbers(offset_18: u32) -> u8 {
    ((offset_18 >> 8) & 0xFF) as u8
}

impl CommonHeaderHoldable for PciBrideDevice {
    fn config_space(&self) -> &ConfigurationSpace {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::pci::config_space::device::pci_bridge_device::convert_to_bus_numbers;

    #[test]
    fn it_sub_numbers() {
        let offset_18 = 0xFF_FC;
        assert_eq!(convert_to_bus_numbers(offset_18), 0xFF);
    }
}
