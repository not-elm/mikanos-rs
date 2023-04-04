use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;

use xhci::ring::trb::event::TransferEvent;

use kernel_lib::serial_println;

use crate::class_driver::interrupt_in::InterruptIn;
use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::class_driver::mouse::mouse_subscribe_driver::MouseSubscriber;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase};
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
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
    pub fn new(interrupters: Vec<InterruptIn<D>>) -> Self {
        Self { interrupters }
    }
}

impl<Memory, Doorbell: 'static, Mouse> Phase<Memory, Doorbell, Mouse> for Phase4<Doorbell>
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
    Mouse: MouseSubscriber + Clone,
{
    fn on_transfer_event_received(
        &mut self,
        _slot: &mut DeviceSlot<Memory, Doorbell>,
        _transfer_event: TransferEvent,
        target_event: TargetEvent,
        _mouse_driver_factory: &MouseDriverFactory<Mouse>,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Memory, Doorbell, Mouse>>>)> {
        if target_event.normal().is_err() {
            return Ok((InitStatus::not(), None));
        }

        for interrupt in self.interrupters.iter_mut() {
            interrupt.interrupter_in()?;
        }

        Ok((InitStatus::not(), None))
    }
}
