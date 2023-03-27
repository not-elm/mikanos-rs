use crate::xhc::registers::internal::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;

pub mod max_device_slots_enabled;

#[derive(Debug)]
pub struct ConfigRegister {
    max_slots_en: MaxDeviceSlotsEnabled,
}

impl ConfigRegister {
    pub fn new(offset: ConfigRegisterOffset) -> Self {
        Self {
            max_slots_en: MaxDeviceSlotsEnabled::new(offset),
        }
    }

    pub fn max_slots_en(&self) -> &MaxDeviceSlotsEnabled {
        &self.max_slots_en
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ConfigRegisterOffset(usize);

impl ConfigRegisterOffset {
    pub fn new(operation_registers_offset: OperationalRegistersOffset) -> Self {
        Self(operation_registers_offset.offset() + 0x38)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
