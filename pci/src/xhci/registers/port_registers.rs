use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhci::registers::port_registers::port::{Port, PortRegisterAddr};

pub mod port;

#[derive(Debug)]
pub struct PortRegisters {
    index: u32,
    operational_registers_offset: OperationalRegistersOffset,
    max_ports: u32,
}

impl PortRegisters {
    pub fn new(operational_registers_offset: OperationalRegistersOffset, max_ports: u32) -> Self {
        Self {
            index: 0,
            operational_registers_offset,
            max_ports,
        }
    }
}

impl Iterator for PortRegisters {
    type Item = Port;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_ports <= self.index {
            return None;
        }

        let port = Port::new(PortRegisterAddr::new(
            self.operational_registers_offset,
            self.index,
        ));
        self.index += 1;
        Some(port)
    }
}
