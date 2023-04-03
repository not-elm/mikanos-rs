use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;

use xhci::context::{Device32Byte, DeviceHandler, Input32Byte, InputHandler};
use xhci::ring::trb::event::TransferEvent;

use crate::class_driver::interrupt_in::InterruptIn;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::descriptor::hid::HidDeviceDescriptors;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase};
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

use super::phase4::Phase4;

pub struct Phase3 {
    hid_device_descriptor_vec: Vec<HidDeviceDescriptors>,
}

impl Phase3 {
    pub fn new(hid_device_descriptor_vec: Vec<HidDeviceDescriptors>) -> Self {
        Self {
            hid_device_descriptor_vec,
        }
    }

    fn interrupters<Memory, Doorbell>(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
    ) -> Vec<InterruptIn<Doorbell>>
    where
        Memory: MemoryAllocatable,
        Doorbell: DoorbellRegistersAccessible,
    {
        self.hid_device_descriptor_vec
            .into_iter()
            .filter_map(|hid| {
                let class_driver = hid.class_driver()?;
                let transfer_ring = slot.try_alloc_transfer_ring(32).ok()?;
                let endpoint_id = hid.endpoint_config().endpoint_id();
                Some(InterruptIn::new(
                    class_driver,
                    slot.id(),
                    endpoint_id,
                    transfer_ring,
                    slot.doorbell(),
                ))
            })
            .collect()
    }
}

impl<Memory, Doorbell: 'static> Phase<Memory, Doorbell> for Phase3
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
{
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        _transfer_event: TransferEvent,
        _target_event: TargetEvent,
    ) -> PciResult<(InitStatus, Box<dyn Phase<Memory, Doorbell>>)> {
        slot.input_context_mut().clear_control();
        slot.copy_device_context_to_input();
        slot.input_context_mut().set_enable_slot_context();

        slot.input_context_mut().slot_mut().set_context_entries(31);
        self.hid_device_descriptor_vec.iter().for_each(|hid| {
            let config = hid.endpoint_config();
            slot.input_context_mut()
                .set_enable_endpoint(DeviceContextIndex::from_endpoint_id(config.endpoint_id()));
            let endpoint_ctx = slot
                .input_context_mut()
                .endpoint_mut_at(config.device_context_index().value());
            config.write_endpoint_context(0, endpoint_ctx);
        });
        let interrupters = self.interrupters(slot);
        Ok((
            InitStatus::initialized(),
            Box::new(Phase4::new(interrupters)),
        ))
    }
}

fn clear_input_context(input_context: &mut Input32Byte) {
    for i in 0..32 {
        input_context.control_mut().clear_add_context_flag(i);
    }
}

fn copy_context(input_context: &mut Input32Byte, device_context: &mut Device32Byte) {
    let device_slot_context = device_context.slot_mut().as_mut();
    let input_slot_context = input_context.device_mut().slot_mut().as_mut();
    unsafe {
        core::ptr::copy(
            device_slot_context.as_ptr(),
            input_slot_context.as_mut_ptr(),
            device_slot_context.len(),
        );
    }
}
