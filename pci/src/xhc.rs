use xhci::ring::trb::event::PortStatusChange;

use kernel_lib::println;
use registers::traits::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;
use registers::traits::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use registers::traits::registers_operation::RegistersOperation;
use registers::traits::usb_command_register_accessible::UsbCommandRegisterAccessible;
use transfer::event::event_ring::EventRing;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::traits::capability_registers_accessible::CapabilityRegistersAccessible;
use crate::xhc::registers::traits::config_register_accessible::ConfigRegisterAccessible;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;
use crate::xhc::transfer::event::event_trb::EventTrb;

pub mod allocator;
pub mod device_manager;
pub mod registers;
pub mod transfer;

pub struct XhcController<T>
where
    T: RegistersOperation
        + InterrupterSetRegisterAccessible
        + PortRegistersAccessible
        + DoorbellRegistersAccessible,
{
    registers: T,
    event_ring: EventRing,
    command_ring: CommandRing,
    _device_context_array: DeviceContextArrayPtr,
}

impl<T> XhcController<T>
where
    T: RegistersOperation
        + CapabilityRegistersAccessible
        + InterrupterSetRegisterAccessible
        + UsbCommandRegisterAccessible
        + DoorbellRegistersAccessible
        + PortRegistersAccessible
        + ConfigRegisterAccessible
        + DeviceContextBaseAddressArrayPointerAccessible,
{
    pub fn new(mut registers: T, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        registers.reset()?;

        registers.write_max_device_slots_enabled(8)?;
        let device_context_array = registers.setup_device_context_array(
            8,
            registers.read_max_scratchpad_buffers_len(),
            allocator,
        )?;

        let command_ring = registers.setup_command_ring(32, allocator)?;

        let (_, event_ring) = registers.setup_event_ring(1, 32, allocator)?;

        registers.run()?;

        Ok(Self {
            registers,
            event_ring,
            command_ring,
            _device_context_array: device_context_array,
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
            self.on_event(event_trb)?;
        }

        Ok(())
    }

    fn on_event(&mut self, event_trb: EventTrb) -> PciResult {
        match event_trb {
            EventTrb::CommandCompletionEvent(completion) => {
                println!("COMMAND! {:?}", completion);
                Ok(())
            }
            EventTrb::PortStatusChangeEvent(port_status) => self.enable_slot(port_status),
            EventTrb::NotSupport { .. } => Ok(()),
        }
    }

    fn enable_slot(&mut self, port_status: PortStatusChange) -> PciResult {
        println!(
            "Receive Port Status Change = {:?} {:x}",
            port_status,
            self.registers.read_event_ring_addr(0)
        );
        let port_id = port_status.port_id();
        self.registers.clear_port_reset_change_at(port_id)?;
        self.command_ring.push_enable_slot(&mut self.registers)?;
        self.event_ring.next_dequeue_pointer(&mut self.registers)
        // self.command_ring.push_no_op(&mut self.registers)
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
