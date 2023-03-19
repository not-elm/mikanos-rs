use crate::error::PciResult;
use crate::xhci::registers::capability_registers::CapabilityRegisters;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhci::registers::operational_registers::OperationalRegisters;

pub mod capability_registers;
pub mod doorbell_registers;
pub mod memory_mapped_addr;
pub mod operational_registers;
pub mod runtime_registers;

#[derive(Debug)]
pub struct Registers {
    pub capability_registers: CapabilityRegisters,
    pub operational_registers: OperationalRegisters,
}

impl Registers {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let capability_registers = CapabilityRegisters::new(mmio_addr)?;
        let operational_registers = OperationalRegisters::new(OperationalRegistersOffset::new(
            mmio_addr,
            capability_registers.cap_length(),
        ))?;
        Ok(Self {
            capability_registers,
            operational_registers,
        })
    }
}
