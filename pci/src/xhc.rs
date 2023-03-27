use core::marker::PhantomData;

use kernel_lib::serial_println;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event_ring::EventRing;
use crate::xhc::transfer::event_ring_table::EventRingTable;

pub mod allocator;
pub mod registers;
pub mod transfer;
pub mod xhci_library_registers;

pub trait XhcRegistersHoldable {
    fn reset(&mut self) -> PciResult;
    fn run(&mut self) -> PciResult;
    fn setup_event_ring(&mut self, allocator: &mut impl MemoryAllocatable)
        -> PciResult<(u64, u64)>;
    fn setup_command_ring(
        &mut self,
        ring_size: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<u64>;

    fn setup_device_context_array(&mut self, allocator: &mut impl MemoryAllocatable) -> PciResult;
}

pub struct XhcController<T>
where
    T: XhcRegistersHoldable,
{
    _p: PhantomData<T>,
    event_ring: EventRing,
    _command_ring: CommandRing,
}

impl<T> XhcController<T>
where
    T: XhcRegistersHoldable,
{
    pub fn new(registers: &mut T, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        registers.reset()?;

        registers.setup_device_context_array(allocator)?;
        let (event_ring_table_addr, event_ring_addr) = registers.setup_event_ring(allocator)?;
        let _event_ring_table = EventRingTable::new(event_ring_table_addr, event_ring_addr)?;
        let event_ring = EventRing::new(event_ring_addr, 32);

        let command_ring = CommandRing::new(registers.setup_command_ring(32, allocator)?, 32);
        registers.run()?;

        Ok(Self {
            _p: PhantomData,
            event_ring,
            _command_ring: command_ring,
        })
    }

    pub fn start_event_pooling(&mut self) {
        loop {
            self.on_event();
        }
    }

    pub fn on_event(&mut self) {
        if let Some(event_trb) = self.event_ring.pop_event_trb() {
            serial_println!("{:?}", event_trb);
        }
    }
}

pub(crate) fn bit_mask_zeros_lower_for(bits: u32, target_value: usize) -> usize {
    let mask = !0 >> (usize::BITS - bits);
    // 下位5Bitsは予約領域
    target_value & !mask
}

#[cfg(test)]
mod tests {
    use crate::xhc::bit_mask_zeros_lower_for;

    #[test]
    fn it_mask_lower_3_bits() {
        assert_eq!(bit_mask_zeros_lower_for(3, 0b1000_0111), 0b1000_0000);
    }

    #[test]
    fn it_mask_lower_5_bits() {
        let addr = 0b1000_0000_0001_1111;
        assert_eq!(bit_mask_zeros_lower_for(5, addr), 0b1000_0000_0000_0000);
    }
}
