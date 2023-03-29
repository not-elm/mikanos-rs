use xhci::context::{DeviceHandler, EndpointType, Input64Byte, InputHandler};

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::transfer_ring::TransferRing;

#[derive(Debug)]
pub struct Device {
    slot_id: u8,
    input_context: Input64Byte,
    device_context: xhci::context::Device64Byte,
    transfer_ring: TransferRing,
}

impl Device {
    pub fn new_with_init_default_control_pipe(
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        let mut me = Self::new(slot_id, allocator)?;
        me.init_slot_context(parent_hub_slot_id, port_speed);
        me.init_default_control_pipe(port_speed);
        Ok(me)
    }

    fn init_slot_context(&mut self, parent_hub_slot_id: u8, port_speed: u8) {
        let slot = self.input_context.device_mut().slot_mut();
        slot.set_parent_hub_slot_id(parent_hub_slot_id);
        slot.set_route_string(0);
        slot.set_context_entries(1);
        slot.set_speed(port_speed);
    }

    fn init_default_control_pipe(&mut self, port_speed: u8) {
        let default_control_pipe = self.device_context.endpoint_mut(1);
        default_control_pipe.set_endpoint_type(EndpointType::Control);
        default_control_pipe.set_max_packet_size(max_packet_size(port_speed));
        default_control_pipe.set_max_burst_size(0);
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_tr_dequeue_pointer(self.transfer_ring.base_address());
        default_control_pipe.set_error_count(3);
    }
    fn new(slot_id: u8, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        let transfer_ring_addr = allocator.try_allocate_trb_ring(32)?;
        let transfer_ring = TransferRing::new(transfer_ring_addr, 32, true);
        Ok(Self {
            slot_id,
            input_context: Input64Byte::new_64byte(),
            device_context: xhci::context::Device64Byte::new_64byte(),
            transfer_ring,
        })
    }
    pub fn device_context_addr(&self) -> u64 {
        (&self.device_context as *const xhci::context::Device64Byte) as u64
    }
    pub fn input_context_addr(&self) -> u64 {
        (&self.input_context as *const xhci::context::Input64Byte) as u64
    }
    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }
}

fn max_packet_size(port_speed: u8) -> u16 {
    match port_speed {
        3 => 64,
        4 => 512,
        _ => 8,
    }
}
