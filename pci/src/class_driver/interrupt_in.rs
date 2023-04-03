use crate::class_driver::ClassDriver;
use crate::error::PciResult;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::device_manager::endpoint_id::EndpointId;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use xhci::ring::trb::transfer::Normal;

pub struct InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    class_driver: Box<dyn ClassDriver>,
    slot_id: u8,
    endpoint_id: EndpointId,
    transfer_ring: TransferRing,
    doorbell: Rc<RefCell<T>>,
}

impl<T> InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(
        class_driver: Box<dyn ClassDriver>,
        slot_id: u8,
        endpoint_id: EndpointId,
        transfer_ring: TransferRing,
        doorbell: &Rc<RefCell<T>>,
    ) -> InterruptIn<T> {
        Self {
            class_driver,
            slot_id,
            endpoint_id,
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

        self.transfer_ring.push(normal.into_raw())?;
        self.notify()
    }

    fn notify(&mut self) -> PciResult {
        self.doorbell.borrow_mut().notify_at(
            self.slot_id as usize,
            DeviceContextIndex::from_endpoint_id(self.endpoint_id).as_u8(),
            0,
        )
    }
}
