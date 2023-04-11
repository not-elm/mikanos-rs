use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;

use xhci::context::EndpointType;
use xhci::ring::trb::event::TransferEvent;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::request_type::RequestType;
use crate::xhc::device_manager::control_pipe::ControlPipeTransfer;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase, DATA_BUFF_SIZE};
use crate::xhc::device_manager::device::phase1::Phase1;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

mod device_slot;
mod phase;
mod phase1;
mod phase2;
mod phase3;
mod phase4;
pub static DUMMY: [u8; 1024] = [0; 1024];
fn DUMMY_ADDR() -> u64 {
    DUMMY.as_ptr() as u64
}
#[repr(C, align(64))]
pub struct Device<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible,
    Memory: MemoryAllocatable,
{
    slot_id: u8,
    phase: Box<dyn Phase<Doorbell, Memory>>,
    doorbell: Rc<RefCell<Doorbell>>,
    slot: DeviceSlot<Memory, Doorbell>,
    device_descriptor_buff: [u8; DATA_BUFF_SIZE],
    _maker: PhantomData<Memory>,
}

impl<Doorbell: 'static, Memory> Device<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible,
    Memory: MemoryAllocatable,
{
    pub fn device_context_addr(&self) -> u64 {
        self.slot
            .device_context()
            .device_context_addr()
    }
    pub fn input_context_addr(&self) -> u64 {
        self.slot
            .input_context()
            .input_context_addr()
    }
    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }

    pub fn new_with_init_default_control_pipe(
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
        doorbell: &Rc<RefCell<Doorbell>>,
        mouse_driver_factory: MouseDriverFactory,
    ) -> PciResult<Self> {
        let mut me = Self::new(slot_id, allocator, doorbell, mouse_driver_factory)?;

        me.slot
            .input_context_mut()
            .set_enable_slot_context();
        me.slot
            .input_context_mut()
            .set_enable_endpoint(DeviceContextIndex::default());

        me.init_slot_context(parent_hub_slot_id, port_speed);
        me.init_default_control_pipe(port_speed);

        Ok(me)
    }

    pub fn start_initialize(&mut self) -> PciResult {
        let buff = self
            .device_descriptor_buff
            .as_mut_ptr();

        const DEVICE_DESCRIPTOR_TYPE: u16 = 1;
        let data_buff_addr = buff as u64;
        let len = self
            .device_descriptor_buff
            .len() as u32;

        self.slot
            .default_control_pipe_mut()
            .control_in()
            .with_data(
                Request::get_descriptor(DEVICE_DESCRIPTOR_TYPE, 0, len as u16),
                data_buff_addr,
                len,
            )
    }

    pub fn on_transfer_event_received(
        &mut self,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult<InitStatus> {
        let (init_status, phase) = self
            .phase
            .on_transfer_event_received(&mut self.slot, transfer_event, target_event)?;
        if let Some(phase) = phase {
            self.phase = phase;
        }

        Ok(init_status)
    }
    pub fn on_endpoints_configured(&mut self) -> PciResult {
        let request_type = RequestType::new()
            .with_ty(1)
            .with_recipient(1);

        self.slot
            .default_control_pipe_mut()
            .control_out()
            .no_data(Request::set_protocol(request_type))
    }

    fn init_slot_context(&mut self, root_port_hub_id: u8, port_speed: u8) {
        let input_context = self.slot.input_context_mut();
        let slot = input_context.slot_mut();
        slot.set_root_hub_port_number(root_port_hub_id);
        slot.set_route_string(0);
        slot.set_context_entries(1);
        slot.set_speed(port_speed);
    }

    fn init_default_control_pipe(&mut self, port_speed: u8) {
        let tr_dequeue_addr = self
            .slot
            .default_control_pipe()
            .transfer_ring_base_addr();
        let control = self.slot.input_context_mut();
        let default_control_pipe = control.endpoint_mut_at(DeviceContextIndex::default().value());

        default_control_pipe.set_endpoint_type(EndpointType::Control);
        default_control_pipe.set_max_packet_size(max_packet_size(port_speed));
        default_control_pipe.set_max_burst_size(0);
        default_control_pipe.set_tr_dequeue_pointer(tr_dequeue_addr);
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_error_count(3);
    }
    fn new(
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
        doorbell: &Rc<RefCell<Doorbell>>,
        mouse_driver_factory: MouseDriverFactory,
    ) -> PciResult<Self> {
        let slot = DeviceSlot::new(slot_id, doorbell, allocator)?;
        let phase = Box::new(Phase1::new(mouse_driver_factory));
        Ok(Self {
            slot_id,
            phase,
            doorbell: Rc::clone(doorbell),
            slot,
            device_descriptor_buff: [0; DATA_BUFF_SIZE],
            _maker: PhantomData,
        })
    }
}

fn max_packet_size(port_speed: u8) -> u16 {
    match port_speed {
        3 => 64,
        4 => 512,
        _ => 8,
    }
}
