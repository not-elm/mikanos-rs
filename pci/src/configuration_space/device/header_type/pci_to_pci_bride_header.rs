use crate::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::configuration_space::device::device_slots::DeviceSlots;
use crate::configuration_space::ConfigurationSpace;

#[derive(Debug)]
pub struct PciToPciBridgeHeader(ConfigurationSpace);

impl PciToPciBridgeHeader {
    pub fn new(config_space: ConfigurationSpace) -> Self {
        Self(config_space)
    }
}

impl PciToPciBridgeHeader {
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

impl CommonHeaderHoldable for PciToPciBridgeHeader {
    fn as_config_space(&self) -> &ConfigurationSpace {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration_space::device::header_type::pci_to_pci_bride_header::convert_to_bus_numbers;

    #[test]
    fn it_sub_numbers() {
        let offset_18 = 0xFF_FC;
        assert_eq!(convert_to_bus_numbers(offset_18), 0xFF);
    }
}
