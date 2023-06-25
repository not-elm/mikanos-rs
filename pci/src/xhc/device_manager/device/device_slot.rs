use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::context::EndpointType;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::ControlPipe;
use crate::xhc::device_manager::device::phase::DATA_BUFF_SIZE;
use crate::xhc::device_manager::device_context::DeviceContext;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::device_manager::input_context::InputContext;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct DeviceSlot<Memory, Doorbell> {
    slot_id: u8,
    default_control_pipe: ControlPipe<Doorbell>,
    input_context: InputContext,
    device_context: DeviceContext,
    data_buff: [u8; DATA_BUFF_SIZE],
    doorbell: Rc<RefCell<Doorbell>>,
    allocator: Rc<RefCell<Memory>>,
}


impl<Memory, Doorbell> DeviceSlot<Memory, Doorbell>
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
{
    pub fn new(
        slot_id: u8,
        doorbell: &Rc<RefCell<Doorbell>>,
        allocator: &Rc<RefCell<Memory>>,
    ) -> PciResult<DeviceSlot<Memory, Doorbell>> {
        let transfer_ring = allocator
            .borrow_mut()
            .try_allocate_trb_ring(32)?;
        let transfer_ring = TransferRing::new(transfer_ring, 32, true);

        let default_control_pipe = ControlPipe::new(
            slot_id,
            DeviceContextIndex::default(),
            doorbell,
            transfer_ring,
        )?;

        Ok(Self {
            slot_id,
            data_buff: [0; DATA_BUFF_SIZE],
            input_context: InputContext::new(),
            device_context: DeviceContext::new(),
            allocator: Rc::clone(allocator),
            doorbell: Rc::clone(doorbell),
            default_control_pipe,
        })
    }


    pub fn init(&mut self) {
        let tr_dequeue_addr = self
            .default_control_pipe()
            .transfer_ring_base_addr();
        let control = self.input_context_mut();
        let default_control_pipe = control.endpoint_mut_at(DeviceContextIndex::default().value());

        default_control_pipe.set_endpoint_type(EndpointType::Control);
        default_control_pipe.set_max_packet_size(64);
        default_control_pipe.set_max_burst_size(0);
        default_control_pipe.set_tr_dequeue_pointer(tr_dequeue_addr);
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_error_count(3);
        self.input_context
            .set_enable_endpoint(DeviceContextIndex::default());
    }

    pub fn id(&self) -> u8 {
        self.slot_id
    }


    pub fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }


    pub fn data_buff_len(&self) -> usize {
        self.data_buff.len()
    }


    pub fn input_context(&self) -> &InputContext {
        &self.input_context
    }


    pub fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }


    pub fn device_context(&self) -> &DeviceContext {
        &self.device_context
    }


    pub fn device_context_mut(&mut self) -> &mut DeviceContext {
        &mut self.device_context
    }


    pub fn copy_device_context_to_input(&mut self) {
        self.input_context
            .copy_from_device_context(self.device_context.slot())
    }


    pub fn default_control_pipe(&self) -> &ControlPipe<Doorbell> {
        &self.default_control_pipe
    }


    pub fn default_control_pipe_mut(&mut self) -> &mut ControlPipe<Doorbell> {
        &mut self.default_control_pipe
    }


    pub fn doorbell(&self) -> &Rc<RefCell<Doorbell>> {
        &self.doorbell
    }


    pub fn try_alloc_transfer_ring(&mut self, ring_size: usize) -> PciResult<TransferRing> {
        let transfer_ring_addr = self
            .allocator
            .borrow_mut()
            .try_allocate_trb_ring(ring_size)?;
        Ok(TransferRing::new(transfer_ring_addr, ring_size, true))
    }
}
