use crate::VolatileAccessible;
use crate::xhc::registers::internal::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use crate::xhc::registers::internal::capability_registers::structural_parameters1::number_of_ports::NumberOfPorts;
use crate::xhc::registers::internal::capability_registers::structural_parameters1::structural_parameters1_field::StructuralParameters1Field;
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;

pub mod number_of_device_slots;
mod number_of_ports;
mod structural_parameters1_field;

#[derive(Debug)]
pub struct StructuralParameters1 {
    max_slots: NumberOfDeviceSlots,
    max_ports: NumberOfPorts,
}

impl StructuralParameters1 {
    pub fn new(offset: StructuralParameters1Offset) -> Self {
        Self {
            max_slots: NumberOfDeviceSlots::new(offset),
            max_ports: NumberOfPorts::new(offset),
        }
    }

    pub fn max_slots(&self) -> &NumberOfDeviceSlots {
        &self.max_slots
    }
    pub fn max_ports(&self) -> u32 {
        self.max_ports.read_volatile()
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StructuralParameters1Offset(usize);

impl StructuralParameters1Offset {
    pub fn new(mmio_addr: MemoryMappedAddr) -> Self {
        Self(mmio_addr.addr() + 0x04)
    }

    fn addr(&self) -> usize {
        self.0
    }
}
