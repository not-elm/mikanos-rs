use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::transfer::Normal;

use crate::class_driver::ClassDriverOperate;
use crate::error::PciResult;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::{ControlPipe, ControlPipeTransfer};
use crate::xhc::device_manager::descriptor::structs::interface_descriptor::InterfaceDescriptor;
use crate::xhc::device_manager::endpoint_config::EndpointConfig;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    slot_id: u8,
    class_driver: Box<dyn ClassDriverOperate>,
    endpoint_config: EndpointConfig,
    transfer_ring: TransferRing,
    interface: InterfaceDescriptor,
    doorbell: Rc<RefCell<T>>,
}

impl<T> InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(
        slot_id: u8,
        class_driver: Box<dyn ClassDriverOperate>,
        endpoint_config: &EndpointConfig,
        transfer_ring: TransferRing,
        doorbell: &Rc<RefCell<T>>,
        interface: InterfaceDescriptor,
    ) -> InterruptIn<T> {
        Self {
            slot_id,
            class_driver,
            endpoint_config: endpoint_config.clone(),
            transfer_ring,
            interface,
            doorbell: Rc::clone(doorbell),
        }
    }
}

impl<T> InterruptIn<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn get_report<Doorbell>(
        &mut self,
        default_control_pipe: &mut ControlPipe<Doorbell>,
    ) -> PciResult
    where
        Doorbell: DoorbellRegistersAccessible,
    {
        self.class_driver
            .on_data_received()?;

        default_control_pipe
            .control_in()
            .with_data(
                Request::get_report(3, 0),
                self.class_driver
                    .data_buff_addr(),
                self.class_driver
                    .data_buff_len(),
            )
    }


    pub fn interrupter_in(&mut self) -> PciResult {
        self.class_driver
            .on_data_received()?;

        let mut normal = Normal::new();
        normal.set_data_buffer_pointer(
            self.class_driver
                .data_buff_addr(),
        );

        normal.set_trb_transfer_length(
            self.class_driver
                .data_buff_len(),
        );

        normal.set_interrupt_on_completion();
        normal.set_interrupt_on_short_packet();

        self.transfer_ring
            .push(normal.into_raw())?;

        self.notify()
    }


    pub fn endpoint_config(&self) -> &EndpointConfig {
        &self.endpoint_config
    }


    pub fn interface_ref(&self) -> &InterfaceDescriptor {
        &self.interface
    }


    pub fn transfer_ring_addr(&self) -> u64 {
        self.transfer_ring
            .base_address()
    }


    pub fn data_buff_addr(&self) -> u64 {
        self.class_driver
            .data_buff_addr()
    }


    #[inline(always)]
    fn notify(&mut self) -> PciResult {
        let endpoint_id = self
            .endpoint_config
            .endpoint_id()
            .value();

        self.doorbell
            .borrow_mut()
            .notify_at(self.slot_id as usize, endpoint_id as u8, 0)
    }
}
