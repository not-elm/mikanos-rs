use kernel_lib::println;

use crate::error::PciResult;
use crate::xhc::registers::internal::capability_registers::CapabilityRegisters;
use crate::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhc::registers::internal::operational_registers::OperationalRegisters;
use crate::xhc::registers::internal::port_registers::PortRegisters;
use crate::xhc::registers::internal::runtime_registers::{
    RuntimeRegisters, RuntimeRegistersOffset,
};
use crate::xhc::RegistersOperation;

pub mod capability_registers;
pub mod doorbell_registers;
pub mod memory_mapped_addr;
pub mod operational_registers;
pub mod port_registers;
pub mod runtime_registers;

#[derive(Debug)]
pub struct Internal {
    mmio_addr: MemoryMappedAddr,
    /// Offset: 0
    capability_registers: CapabilityRegisters,
    /// Offset: CapLength Byte
    operational_registers: OperationalRegisters,
    /// Offset: RuntimeRegistersSpaceOffset
    runtime_registers: RuntimeRegisters,
}

impl RegistersOperation for Internal {
    fn reset(&mut self) -> PciResult {
        self.operational_registers.reset_host_controller();
        Ok(())
    }

    fn run(&mut self) -> PciResult {
        self.operational_registers.run_host_controller();
        self.port_registers()
            .filter(|port| port.is_connect())
            .for_each(|port| {
                port.reset();
                println!("Port = {:?}", port);
            });

        Ok(())
    }
}

impl Internal {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let capability_registers = CapabilityRegisters::new(mmio_addr)?;
        let operational_registers = OperationalRegisters::new(OperationalRegistersOffset::new(
            mmio_addr,
            capability_registers.cap_length(),
        ))?;
        let runtime_registers = RuntimeRegisters::new(RuntimeRegistersOffset::new(
            mmio_addr,
            capability_registers.rts_off(),
        ));

        Ok(Self {
            mmio_addr,
            capability_registers,
            operational_registers,
            runtime_registers,
        })
    }

    pub fn port_registers(&self) -> PortRegisters {
        PortRegisters::new(
            OperationalRegistersOffset::new(self.mmio_addr, self.capability_registers.cap_length()),
            self.capability_registers.hcs_params1().max_ports(),
        )
    }

    pub fn runtime_registers(&self) -> &RuntimeRegisters {
        &self.runtime_registers
    }
}
