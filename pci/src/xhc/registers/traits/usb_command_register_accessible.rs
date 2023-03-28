use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::command_ring::CommandRing;

pub trait UsbCommandRegisterAccessible {
    fn write_command_ring_addr(&mut self, command_ring_addr: u64) -> PciResult;

    fn setup_command_ring(
        &mut self,
        command_ring_size: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<CommandRing> {
        let command_ring_addr = allocator.try_allocate_trb_ring(command_ring_size)?;
        let command_ring = CommandRing::new(command_ring_addr, command_ring_size);
        self.write_command_ring_addr(command_ring_addr)?;
        Ok(command_ring)
    }
}
