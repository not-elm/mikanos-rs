use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::transfer::Normal;

use kernel_lib::serial_println;

use crate::class_driver::{ClassDriver, ClassDriverOperate};
use crate::error::PciResult;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::device_manager::endpoint_config::EndpointConfig;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    slot_id: u8,
    class_driver: ClassDriver,
    endpoint_config: EndpointConfig,
    transfer_ring: TransferRing,
    doorbell: Rc<RefCell<T>>,
}

impl<T> InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(
        slot_id: u8,
        class_driver: ClassDriver,
        endpoint_config: &EndpointConfig,
        transfer_ring: TransferRing,
        doorbell: &Rc<RefCell<T>>,
    ) -> InterruptIn<T> {
        Self {
            slot_id,
            class_driver,
            endpoint_config: endpoint_config.clone(),
            transfer_ring,
            doorbell: Rc::clone(doorbell),
        }
    }
}

impl<T> InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn interrupter_in(&mut self) -> PciResult {
        self.class_driver.on_data_received()?;
        let mut normal = Normal::new();
        normal.set_data_buffer_pointer(self.class_driver.data_buff_addr());

        normal.set_interrupt_on_short_packet();
        normal.set_trb_transfer_length(self.class_driver.data_buff_len());

        normal.set_interrupt_on_completion();
        serial_println!("{:?}", normal);
        self.transfer_ring.push(normal.into_raw())?;
        self.notify()
    }
    pub fn endpoint_config(&self) -> &EndpointConfig {
        &self.endpoint_config
    }
    pub fn transfer_ring_addr(&self) -> u64 {
        self.transfer_ring.base_address()
    }
    fn notify(&mut self) -> PciResult {
        self.doorbell.borrow_mut().notify_at(
            self.slot_id as usize,
            DeviceContextIndex::from_endpoint_id(self.endpoint_config.endpoint_id()).as_u8(),
            0,
        )
    }
}
