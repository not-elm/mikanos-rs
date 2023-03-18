pub mod capability_registers;
pub mod doorbell_registers;
pub mod memory_mapped_addr;
pub mod operational_registers;
pub mod runtime_registers;

pub struct Registers {}

impl Registers {
    pub fn new(_mmio_base_addr: usize) -> Self {
        Self {}
    }
}
