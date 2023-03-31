use bitfield_struct::bitfield;
use xhci::context::{Device32Byte, EndpointType, Input32Byte, InputHandler};
use xhci::ring::trb::transfer::{SetupStage, StatusStage, TransferType};

use kernel_lib::println;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::initialize_phase::InitializePhase;
use crate::xhc::transfer::transfer_ring::TransferRing;

#[repr(C, align(64))]
#[derive(Debug)]
pub struct Device {
    input_context: InputContext,
    slot_id: u8,
    device_context: DeviceContext,
    transfer_ring: TransferRing,
    phase: InitializePhase,
}

#[bitfield(u8)]
pub struct RequestType {
    #[bits(5)]
    pub recipient: u8,
    #[bits(2)]
    pub ty: u8,
    pub direction: bool,
}

#[repr(C, align(64))]
#[derive(Debug)]
struct InputContext(xhci::context::Input32Byte);

#[repr(C, align(64))]
#[derive(Debug)]
struct DeviceContext(xhci::context::Device32Byte);

impl Device {
    pub fn new_with_init_default_control_pipe(
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        let mut me = Self::new(slot_id, allocator)?;

        me.set_enable_slot_context();
        me.set_enable_device_context(calc_device_context_index(0, false));

        me.init_slot_context(parent_hub_slot_id, port_speed);
        me.init_default_control_pipe(port_speed);

        Ok(me)
    }
    pub fn setup(&mut self, status_stage: StatusStage) -> PciResult {
        self.phase = InitializePhase::Phase2;
        let mut status = xhci::ring::trb::transfer::StatusStage::new();
        status.set_direction();
        status.set_interrupt_on_completion();

        // Config
        self.transfer_ring
            .push(make_setup_stage(2, 0, 0).into_raw())?;
        self.transfer_ring.push(status.into_raw())
    }

    pub fn control_in_setup_data(&mut self) -> PciResult {
        self.phase = InitializePhase::Phase1;
        let mut status = xhci::ring::trb::transfer::StatusStage::new();
        status.set_direction();
        status.set_interrupt_on_completion();

        self.transfer_ring
            .push(make_setup_stage(1, 0, 0).into_raw())?;
        self.transfer_ring.push(status.into_raw())
    }

    fn init_slot_context(&mut self, root_port_hub_id: u8, port_speed: u8) {
        let slot = self.input_context.0.device_mut().slot_mut();
        slot.set_root_hub_port_number(root_port_hub_id);
        slot.set_route_string(0);
        slot.set_context_entries(1);
        slot.set_speed(port_speed);
    }

    fn init_default_control_pipe(&mut self, port_speed: u8) {
        let default_control_pipe = self
            .input_context
            .0
            .device_mut()
            .endpoint_mut(calc_device_context_index(0, false));

        default_control_pipe.set_endpoint_type(EndpointType::Control);
        default_control_pipe.set_max_packet_size(max_packet_size(port_speed));
        default_control_pipe.set_max_burst_size(0);
        default_control_pipe.set_tr_dequeue_pointer(self.transfer_ring.base_address());
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_error_count(3);
    }
    fn new(slot_id: u8, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        let transfer_ring_addr = allocator.try_allocate_trb_ring(32)?;

        let transfer_ring = TransferRing::new(transfer_ring_addr, 32, true);
        Ok(Self {
            slot_id,
            input_context: InputContext(Input32Byte::default()),
            device_context: DeviceContext(Device32Byte::default()),
            transfer_ring,
            phase: InitializePhase::NotPrepared,
        })
    }
    pub fn device_context_addr(&self) -> u64 {
        (&self.device_context.0 as *const xhci::context::Device32Byte) as u64
    }
    pub fn input_context_addr(&self) -> u64 {
        (&self.input_context.0 as *const Input32Byte) as u64
    }
    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }

    fn set_enable_slot_context(&mut self) {
        self.input_context.0.control_mut().set_add_context_flag(0);
    }
    fn set_enable_device_context(&mut self, device_context_index: usize) {
        self.input_context
            .0
            .control_mut()
            .set_add_context_flag(device_context_index);
    }
}

fn make_setup_stage(descriptor_type: u16, descriptor_index: u16, length: u16) -> SetupStage {
    // // Standard
    let mut setup = xhci::ring::trb::transfer::SetupStage::new();
    setup.set_request_type(RequestType::new().with_direction(true).into());
    setup.set_request(6);
    setup.set_value(descriptor_type << 8 | descriptor_index);
    setup.set_index(0);
    setup.set_length(length);
    setup.set_transfer_type(TransferType::No);
    setup
}

fn max_packet_size(port_speed: u8) -> u16 {
    println!("Port Speed = {}", port_speed);
    match port_speed {
        3 => 64,
        4 => 512,
        _ => 8,
    }
}

fn calc_device_context_index(endpoint_index: usize, dir_in: bool) -> usize {
    2 * endpoint_index
        + if endpoint_index == 0 {
            1
        } else {
            if dir_in {
                1
            } else {
                0
            }
        }
}
