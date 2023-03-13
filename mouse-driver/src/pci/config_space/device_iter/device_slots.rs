use kernel_lib::println;

use crate::pci::config_space::access::ConfigurationSpace;
use crate::pci::config_space::device::PciDevice;

#[derive(Debug)]
pub struct DeviceSlots {
    bus: u8,
    device_slot: u8,
}

impl Iterator for DeviceSlots {
    type Item = PciDevice;

    fn next(&mut self) -> Option<Self::Item> {
        self.a()
    }
}

impl DeviceSlots {
    fn a(&mut self) -> Option<PciDevice> {
        const DEVICE_SLOT_SIZE: u8 = 32;
        if DEVICE_SLOT_SIZE <= self.device_slot {
            return None;
        }

        let config = ConfigurationSpace::try_new(self.bus, self.device_slot, 0);
        let device = config.and_then(|c| c.cast_device());
        self.device_slot += 1;
        if let Some(device) = device {

            return Some(device);
        }

        self.a()
    }
    pub(crate) fn new(bus: u8) -> Self {
        Self {
            bus,
            device_slot: 0,
        }
    }
    fn exit_if_max_slots(&mut self) -> Option<()> {
        const DEVICE_SLOT_SIZE: u8 = 32;
        if DEVICE_SLOT_SIZE <= self.device_slot {
            return None;
        }

        Some(())
    }
}
