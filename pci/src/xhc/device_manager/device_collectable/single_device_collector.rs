use crate::error::{DeviceContextReason, PciError, PciResult};
use crate::xhc::device_manager::device::Device;
use crate::xhc::device_manager::device_collectable::DeviceCollectable;

#[derive(Debug)]
pub struct SingleDeviceCollector {
    device_slots: u8,
    device: Option<Device>,
}

impl SingleDeviceCollector {
    pub fn new(device_slots: u8) -> Self {
        Self {
            device_slots,
            device: None,
        }
    }

    fn check_specify_slot_id(&self, slot_id: u8) -> PciResult {
        if self.device_slots - 1 < slot_id {
            Err(PciError::FailedOperateDeviceContext(
                DeviceContextReason::ExceedMasSlots {
                    max_slots: self.device_slots,
                    specified_slot_id: slot_id,
                },
            ))
        } else {
            Ok(())
        }
    }
}

impl DeviceCollectable for SingleDeviceCollector {
    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device> {
        self.check_specify_slot_id(slot_id).ok()?;

        self.device.as_mut().and_then(|device| {
            if device.slot_id() == slot_id {
                Some(device)
            } else {
                None
            }
        })
    }

    fn new_set_at(&mut self, port_id: u8, port_speed: u8, slot_id: u8) -> PciResult {
        self.check_specify_slot_id(slot_id)?;
        self.device = Some(Device::new_with_init_slot_context(
            port_id, port_speed, slot_id,
        ));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::xhc::device_manager::device_collectable::single_device_collector::SingleDeviceCollector;
    use crate::xhc::device_manager::device_collectable::DeviceCollectable;

    #[test]
    fn it_set_device_into_slot_id0() {
        let mut ptr = SingleDeviceCollector::new(1);
        assert!(ptr.new_set_at(0, 0, 0).is_ok());
    }

    #[test]
    fn it_get_device_from_slot_id0() {
        let mut ptr = SingleDeviceCollector::new(1);
        assert!(ptr.new_set_at(0, 0, 0).is_ok());
        assert_eq!(0, ptr.mut_at(0).unwrap().slot_id());
    }

    #[test]
    fn it_set_device_into_slot_id20_and_100() {
        let mut ptr = SingleDeviceCollector::new(101);
        assert!(ptr.new_set_at(0, 0, 100).is_ok());

        assert_eq!(100, ptr.mut_at(100).unwrap().slot_id());
    }

    #[test]
    fn it_failed_set_device_when_over_slots() {
        let mut ptr = SingleDeviceCollector::new(1);
        assert!(ptr.new_set_at(0, 0, 1).is_err());
    }

    #[test]
    fn it_failed_get_device_when_over_slots() {
        let mut ptr = SingleDeviceCollector::new(1);
        assert!(ptr.new_set_at(0, 0, 0).is_ok());
        assert!(ptr.mut_at(1).is_none());
    }

    #[test]
    fn it_failed_when_get_uninit() {
        let mut ptr = SingleDeviceCollector::new(1);

        assert!(ptr.mut_at(0).is_none());
    }
}
