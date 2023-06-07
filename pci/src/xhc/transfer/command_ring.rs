use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::command::ConfigureEndpoint;

use crate::error::PciResult;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct CommandRing<T> {
    transfer_ring: TransferRing,
    doorbell: Rc<RefCell<T>>,
}


impl<T> CommandRing<T>
    where
        T: DoorbellRegistersAccessible,
{
    pub fn new(ring_ptr_addr: u64, ring_size: usize, doorbell: &Rc<RefCell<T>>) -> Self {
        Self {
            transfer_ring: TransferRing::new(ring_ptr_addr, ring_size, true),
            doorbell: Rc::clone(doorbell),
        }
    }


    pub fn push_no_op(&mut self) -> PciResult {
        self.transfer_ring
            .push(xhci::ring::trb::command::Noop::new().into_raw())?;
        self.notify()
    }


    pub fn push_configure_endpoint(
        &mut self,
        input_context_addr: u64,
        slot_id: u8,
    ) -> PciResult {
        let mut configure_endpoint_trb = ConfigureEndpoint::new();
        configure_endpoint_trb.set_slot_id(slot_id);
        configure_endpoint_trb.set_input_context_pointer(input_context_addr);

        self.transfer_ring
            .push(configure_endpoint_trb.into_raw())?;
        self.notify()
    }


    pub fn push_address_command(&mut self, input_context_addr: u64, slot_id: u8) -> PciResult {
        let mut address_command = xhci::ring::trb::command::AddressDevice::new();
        address_command.set_input_context_pointer(input_context_addr);
        address_command.set_slot_id(slot_id);

        self.transfer_ring
            .push(address_command.into_raw())?;
        self.notify()
    }


    pub fn push_enable_slot(&mut self) -> PciResult {
        self.transfer_ring
            .push(xhci::ring::trb::command::EnableSlot::new().into_raw())?;
        self.notify()
    }


    fn notify(&mut self) -> PciResult {
        self.doorbell
            .borrow_mut()
            .notify_at(0, 0, 0)
    }
}
