use alloc::boxed::Box;
use alloc::vec::Vec;

use xhci::ring::trb::event::TransferEvent;

use crate::class_driver::interrupt_in::InterruptIn;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase};
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

pub struct Phase4<Doorbell>
where
    Doorbell: DoorbellRegistersAccessible,
{
    interrupters: Vec<InterruptIn<Doorbell>>,
}

impl<D> Phase4<D>
where
    D: DoorbellRegistersAccessible,
{
    pub const fn new(interrupters: Vec<InterruptIn<D>>) -> Self {
        Self { interrupters }
    }
}


impl<Doorbell: 'static, Memory> Phase<Doorbell, Memory> for Phase4<Doorbell>
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
{
    fn on_transfer_event_received(
        &mut self,
        _slot: &mut DeviceSlot<Memory, Doorbell>,
        _transfer_event: TransferEvent,
        _target_event: TargetEvent,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Doorbell, Memory>>>)> {
        for interrupt in self.interrupters.iter_mut() {
            interrupt
                .interrupter_in()
                .unwrap();
        }

        Ok((InitStatus::not(), None))
    }


    fn interface_nums(&self) -> Option<Vec<u8>> {
        Some(
            self.interrupters
                .iter()
                .map(|i| i.interface_ref().interface_id)
                .collect(),
        )
    }
}
