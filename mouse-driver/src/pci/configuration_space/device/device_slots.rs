use crate::pci::configuration_space::device::function::Function;
use crate::pci::configuration_space::ConfigurationSpace;

#[derive(Debug)]
pub struct DeviceSlots {
    bus: u8,
    device_slot: u8,
}

impl Iterator for DeviceSlots {
    type Item = Function;

    fn next(&mut self) -> Option<Self::Item> {
        const DEVICE_SLOT_SIZE: u8 = 32;
        if DEVICE_SLOT_SIZE <= self.device_slot {
            return None;
        }

        let config = ConfigurationSpace::try_new(self.bus, self.device_slot, 0);
        let device = config.map(|c| c.cast_device());
        self.device_slot += 1;
        if let Some(device) = device {
            return Some(device);
        }

        self.next()
    }
}

impl DeviceSlots {
    pub(crate) fn new(bus: u8) -> Self {
        Self {
            bus,
            device_slot: 0,
        }
    }
}
