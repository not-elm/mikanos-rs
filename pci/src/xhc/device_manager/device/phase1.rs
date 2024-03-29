use alloc::boxed::Box;
use alloc::vec::Vec;

use xhci::ring::trb::event::TransferEvent;

use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::mouse::driver::MouseDriver;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::ControlPipeTransfer;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase};
use crate::xhc::device_manager::device::phase2::Phase2;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

/// コンフィグディスクリプタを取得します。
pub struct Phase1 {
    mouse: MouseDriver,
    keyboard: KeyboardDriver,
}


impl Phase1 {
    pub const fn new(mouse: MouseDriver, keyboard: KeyboardDriver) -> Phase1 {
        Self { mouse, keyboard }
    }
}


impl<Doorbell, Memory> Phase<Doorbell, Memory> for Phase1
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible + 'static,
{
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        _transfer_event: TransferEvent,
        _target_event: TargetEvent,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Doorbell, Memory>>>)> {
        const CONFIGURATION_TYPE: u16 = 2;

        let data_buff_addr = slot.data_buff_addr();
        let len = slot.data_buff_len() as u32;
        let request = Request::get_descriptor(CONFIGURATION_TYPE, 0, len as u16);
        slot.default_control_pipe_mut()
            .control_in()
            .with_data(request, data_buff_addr, len)?;

        Ok((
            InitStatus::not(),
            Some(Box::new(Phase2::new(
                self.mouse.clone(),
                self.keyboard.clone(),
            ))),
        ))
    }


    fn interface_nums(&self) -> Option<Vec<u8>> {
        None
    }
}
