use alloc::rc::Rc;
use core::cell::RefCell;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::command_ring::CommandRing;

pub trait UsbCommandRegisterAccessible {
    fn write_command_ring_addr(&mut self, command_ring_addr: u64) -> PciResult;
}


pub(crate) fn setup_command_ring<T>(
    registers: &mut Rc<RefCell<T>>,
    command_ring_size: usize,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<CommandRing<T>>
    where
        T: UsbCommandRegisterAccessible + DoorbellRegistersAccessible,
{
    let command_ring_addr = allocator.try_allocate_trb_ring(command_ring_size)?;
    let command_ring =
        CommandRing::new(command_ring_addr & !0b111111, command_ring_size, registers);
    registers
        .borrow_mut()
        .write_command_ring_addr(command_ring_addr)?;

    Ok(command_ring)
}
