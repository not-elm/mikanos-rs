use alloc::boxed::Box;

use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::class_driver::mouse::mouse_subscribe_driver::MouseSubscriber;
use xhci::ring::trb::event::TransferEvent;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

pub(crate) const DATA_BUFF_SIZE: usize = 256;

/// Configure Commandを送信するか
pub struct InitStatus(bool);

impl InitStatus {
    pub fn new(is_initialized: bool) -> Self {
        Self(is_initialized)
    }
    pub fn not() -> Self {
        Self::new(false)
    }

    pub fn initialized() -> Self {
        Self::new(true)
    }
    pub fn is_initialised(&self) -> bool {
        self.0
    }
}

pub trait Phase<Memory, Doorbell, Mouse>
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
    Mouse: MouseSubscriber + Clone,
{
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
        mouse_driver_factory: &MouseDriverFactory<Mouse>,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Memory, Doorbell, Mouse>>>)>;
}
