use core::marker::PhantomData;

use kernel_lib::serial_println;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;
use crate::xhc::registers::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use crate::xhc::registers::registers_operation::RegistersOperation;
use crate::xhc::registers::usb_command_register_accessible::UsbCommandRegisterAccessible;
use crate::xhc::transfer::command_ring::CommandRing;
use transfer::event::event_ring_segment::EventRingSegment;

pub mod allocator;
pub mod registers;
pub mod transfer;

pub struct XhcController<T>
where
    T: RegistersOperation + InterrupterSetRegisterAccessible,
{
    registers: T,
    event_ring: EventRingSegment,
    _command_ring: CommandRing,
}

impl<T> XhcController<T>
where
    T: RegistersOperation
        + InterrupterSetRegisterAccessible
        + UsbCommandRegisterAccessible
        + DeviceContextBaseAddressArrayPointerAccessible,
{
    pub fn new(mut registers: T, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        registers.reset()?;

        let device_context_array_addr = allocator.try_allocate_trb_ring(1024)?;
        registers.write_device_context_array_addr(device_context_array_addr)?;

        let (_, event_ring) = registers.setup_event_ring(1, 32, allocator)?;

        let command_ring_addr = allocator.try_allocate_trb_ring(32)?;
        let command_ring = CommandRing::new(command_ring_addr, 32);
        registers.write_command_ring_addr(command_ring_addr)?;

        registers.run()?;

        Ok(Self {
            registers,
            event_ring,
            _command_ring: command_ring,
        })
    }

    pub fn start_event_pooling(&mut self) -> PciResult {
        loop {
            self.check_event()?;
        }
    }

    pub fn check_event(&mut self) -> PciResult {
        if let Some(event_trb) = self
            .event_ring
            .read_event_trb(self.registers.read_event_ring_addr(0))
        {
            serial_println!("{:?}", event_trb);
            self.event_ring.next_dequeue_pointer(&mut self.registers)?;
        }

        Ok(())
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
