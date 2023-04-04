use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;

use xhci::context::{Device32Byte, DeviceHandler, Input32Byte, InputHandler};
use xhci::ring::trb::event::TransferEvent;

use crate::class_driver::interrupt_in::InterruptIn;
use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::class_driver::mouse::mouse_subscribe_driver::MouseSubscriber;
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

    fn interrupters<Memory, Doorbell, Mouse>(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        mouse_driver_factory: &MouseDriverFactory<Mouse>,
    ) -> Vec<InterruptIn<Doorbell>>
    where
        Memory: MemoryAllocatable,
        Doorbell: DoorbellRegistersAccessible,
        Mouse: MouseSubscriber + Clone,
    {
        self.hid_device_descriptor_vec
            .iter()
            .filter_map(|hid| {
                let class_driver = hid.class_driver(mouse_driver_factory)?;
                let transfer_ring = slot.try_alloc_transfer_ring(32).ok()?;

                Some(InterruptIn::new(
                    slot.id(),
                    class_driver,
                    &hid.endpoint_config(),
                    transfer_ring,
                    slot.doorbell(),
                ))
            })
            .collect()
    }
}

impl<Memory, Doorbell: 'static, Mouse> Phase<Memory, Doorbell, Mouse> for Phase3
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
    Mouse: MouseSubscriber + Clone,
{
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        _transfer_event: TransferEvent,
        _target_event: TargetEvent,
        mouse_driver_factory: &MouseDriverFactory<Mouse>,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Memory, Doorbell, Mouse>>>)> {
        slot.input_context_mut().clear_control();
        slot.copy_device_context_to_input();
        slot.input_context_mut().set_enable_slot_context();

        slot.input_context_mut().slot_mut().set_context_entries(31);
        let interrupters = self.interrupters(slot);
        interrupters.iter().for_each(|interrupt| {
            let config = interrupt.endpoint_config();
            slot.input_context_mut()
                .set_enable_endpoint(DeviceContextIndex::from_endpoint_id(config.endpoint_id()));
            let endpoint_ctx = slot
                .input_context_mut()
                .endpoint_mut_at(config.device_context_index().value());
            config.write_endpoint_context(interrupt.transfer_ring_addr(), endpoint_ctx);
        });

        Ok((
            InitStatus::initialized(),
            Some(Box::new(Phase4::new(interrupters))),
        ))
    }
}
