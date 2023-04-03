use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::ptr::null_mut;

use bitfield_struct::bitfield;
use xhci::context::{
    Device32Byte, DeviceHandler, EndpointState, EndpointType, Input32Byte, InputHandler,
};
use xhci::ring::trb::event::TransferEvent;
use xhci::ring::trb::transfer::Direction::{In, Out};
use xhci::ring::trb::transfer::{DataStage, SetupStage, StatusStage, TransferType};

use kernel_lib::println;

use crate::class_driver::mouse::Mouse;
use crate::error::{PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::ControlPipe;
use crate::xhc::device_manager::descriptor::configuration_descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptors,
};
use crate::xhc::device_manager::descriptor::descriptor::UsbDescriptor;
use crate::xhc::device_manager::descriptor::device_descriptor::DeviceDescriptor;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::device_manager::endpoint_config::EndpointConfig;
use crate::xhc::device_manager::endpoint_id::EndpointId;
use crate::xhc::device_manager::initialize_phase::InitializePhase;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::event_trb::TargetEvent;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub mod phase0;

const DATA_BUFF: usize = 256;

#[repr(C, align(64))]
pub struct Device<T>
where
    T: DoorbellRegistersAccessible,
{
    input_context: InputContext,
    slot_id: u8,
    device_context: DeviceContext,
    default_control_pipe: ControlPipe<T>,
    phase: InitializePhase,
    data_buff: [u8; DATA_BUFF],
    hid_buff: [u8; 1024],
    doorbell: Rc<RefCell<T>>,
    endpoint_configs: Vec<EndpointConfig>,
    setup_stage: Option<SetupStage>,
    mouse: Mouse,
    interface_num: u16,
    ring2: TransferRing,
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
struct InputContext(Input32Byte);

#[repr(C, align(64))]
#[derive(Debug)]
struct DeviceContext(Device32Byte);

impl<T> Device<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new_with_init_default_control_pipe(
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
        doorbell: &Rc<RefCell<T>>,
    ) -> PciResult<Self> {
        let mut me = Self::new(slot_id, allocator, doorbell)?;

        me.set_enable_slot_context();
        me.set_enable_endpoint(DeviceContextIndex::new(0, false));

        me.init_slot_context(parent_hub_slot_id, port_speed);
        me.init_default_control_pipe(port_speed);

        Ok(me)
    }
    pub fn is_init(&self) -> bool {
        matches!(self.phase, InitializePhase::Completed)
    }

    pub fn start_initialize(&mut self) -> PciResult {
        self.phase = InitializePhase::Phase1;
        let buff = self.data_buff.as_mut_ptr();
        const DEVICE_DESCRIPTOR_TYPE: u16 = 1;
        self.get_descriptor(
            EndpointId::from_endpoint_num(0, true),
            DEVICE_DESCRIPTOR_TYPE,
            0,
            buff,
            self.data_buff.len() as u16,
        )
    }

    pub fn on_transfer_event_received(&mut self, transfer_event: TransferEvent) -> PciResult {
        let prev = self.setup_stage.ok_or(PciError::NullPointer)?;
        let mut setup_data = SetupStage::new();
        setup_data.set_request_type(prev.request_type());
        setup_data.set_request(prev.request());
        setup_data.set_value(prev.value());
        setup_data.set_index(prev.index());
        setup_data.set_length(prev.length());

        match TargetEvent::new(transfer_event.trb_pointer()).ok_or(PciError::NullPointer)? {
            TargetEvent::Normal(normal) => Ok(()),
            TargetEvent::StatusStage(status) => self.on_control_completed(
                EndpointId::from_addr(transfer_event.endpoint_id() as usize),
                setup_data,
                null_mut(),
                0,
            ),
            TargetEvent::DataStage(data_stage) => {
                let data_stage_buff = data_stage.data_buffer_pointer() as *mut u8;
                let residual_length = transfer_event.trb_transfer_length();
                self.on_control_completed(
                    EndpointId::from_addr(transfer_event.endpoint_id() as usize),
                    setup_data,
                    data_stage_buff,
                    (data_stage.trb_transfer_length() - residual_length) as u16,
                )
            }
        }
    }
    pub fn on_endpoints_configured(&mut self) -> PciResult {
        let mut setup = SetupStage::new();
        let request_type = RequestType::new().with_ty(1).with_recipient(1);

        setup.set_value(0);
        setup.set_request_type(request_type.0);
        setup.set_request(11);
        setup.set_index(0);

        self.control_out(EndpointId::from_endpoint_num(0, true), setup, null_mut(), 0)
    }
    fn on_control_completed(
        &mut self,
        ep_id: EndpointId,
        setup_data: SetupStage,
        buff: *mut u8,
        len: u16,
    ) -> PciResult {
        match self.phase {
            InitializePhase::Phase1 => self.initialize_phase1(buff, len),
            InitializePhase::Phase2 => self.initialize_phase2(buff, len),
            InitializePhase::Phase3 => {
                self.phase = InitializePhase::Completed;
                self.configure_endpoints();
                Ok(())
            }
            InitializePhase::Completed => self.interrupter_in(ep_id),
            _ => Err(PciError::NullPointer),
        }
    }

    pub fn interrupter_in(&mut self, ep_id: EndpointId) -> PciResult {
        self.mouse.on_data_received()?;
        self.phase = InitializePhase::Finish;
        let mut normal = xhci::ring::trb::transfer::Normal::new();
        normal.set_data_buffer_pointer(self.mouse.data_buff_addr());

        normal.set_interrupt_on_short_packet();
        normal.set_trb_transfer_length(3);

        let dci = DeviceContextIndex::from_endpoint_id(ep_id);

        normal.set_interrupt_on_completion();

        self.ring2.push(normal.into_raw())?;
        self.notify(DeviceContextIndex::from_dci(3))
    }
    fn initialize_phase1(&mut self, buff: *mut u8, len: u16) -> PciResult {
        let device_descriptor = unsafe { *(buff as *const DeviceDescriptor) };
        self.phase = InitializePhase::Phase2;

        const CONFIGURATION_TYPE: u16 = 2;

        let data_buff = self.data_buff.as_mut_ptr();
        self.get_descriptor(
            EndpointId::from_endpoint_num(0, true),
            CONFIGURATION_TYPE,
            0,
            data_buff,
            self.data_buff.len() as u16,
        )
    }

    fn initialize_phase2(&mut self, buff: *mut u8, len: u16) -> PciResult {
        let conf_desc = unsafe { *(buff as *const ConfigurationDescriptor) };
        self.phase = InitializePhase::Phase3;

        let descriptors = ConfigurationDescriptors::new(buff, len as usize);

        let mut interfaces = descriptors.skip_while(|d| d.interface().is_none());
        let interface = interfaces.next().unwrap();
        self.interface_num = interface.interface().unwrap().interface_id as u16;
        for endpoint in interfaces.filter_map(|d| {
            if let UsbDescriptor::Endpoint(endpoint) = d {
                Some(endpoint)
            } else {
                None
            }
        }) {
            self.endpoint_configs.push(EndpointConfig::new(endpoint));
        }

        self.set_configuration(
            EndpointId::from_endpoint_num(0, true),
            conf_desc.configuration_value as u16,
        )
    }
    fn configure_endpoints(&mut self) {
        for i in 0..32 {
            self.input_context.0.control_mut().clear_add_context_flag(i);
        }

        self.copy_context();

        self.set_enable_slot_context();
        let slot_context = self.input_context.0.device_mut().slot_mut();
        slot_context.set_context_entries(31);

        let endpoint_config = self.endpoint_configs.pop().unwrap();
        self.set_enable_endpoint(endpoint_config.device_context_index());

        let endpoint_ctx = self
            .input_context
            .0
            .device_mut()
            .endpoint_mut(endpoint_config.device_context_index().value());
        endpoint_ctx.set_max_packet_size(endpoint_config.max_packet_size());
        endpoint_ctx.set_interval(endpoint_config.interval() - 1);
        endpoint_ctx.set_average_trb_length(1);
        endpoint_ctx.set_endpoint_state(EndpointState::Running);
        endpoint_ctx.set_error_count(3);
        endpoint_ctx.set_tr_dequeue_pointer(self.ring2.base_address());
        endpoint_ctx.set_endpoint_type(EndpointType::InterruptIn);
        endpoint_ctx.set_dequeue_cycle_state();
        endpoint_ctx.set_mult(0);
        endpoint_ctx.set_max_primary_streams(0);
    }

    fn copy_context(&mut self) {
        let device_slot_context = self.device_context.0.slot_mut().as_mut();
        let input_slot_context = self.input_context.0.device_mut().slot_mut().as_mut();
        unsafe {
            core::ptr::copy(
                device_slot_context.as_ptr(),
                input_slot_context.as_mut_ptr(),
                device_slot_context.len(),
            );
        }
    }

    fn set_configuration(&mut self, ep_id: EndpointId, config_value: u16) -> PciResult {
        let mut setup_data = SetupStage::new();

        const CONFIGURATION: u8 = 9;
        setup_data.set_request(CONFIGURATION);
        setup_data.set_value(config_value);
        setup_data.set_index(0);
        setup_data.set_length(0);

        self.control_out(ep_id, setup_data, null_mut(), 0)
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
            .endpoint_mut(DeviceContextIndex::new(0, false).value());

        default_control_pipe.set_endpoint_type(EndpointType::Control);
        default_control_pipe.set_max_packet_size(max_packet_size(port_speed));
        default_control_pipe.set_max_burst_size(0);
        default_control_pipe.set_tr_dequeue_pointer(self.default_control_pipe.);
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_error_count(3);
    }
    fn new(
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
        doorbell: &Rc<RefCell<T>>,
    ) -> PciResult<Self> {
        Ok(Self {
            slot_id,
            input_context: InputContext(Input32Byte::default()),
            device_context: DeviceContext(Device32Byte::default()),
            default_control_pipe: ControlPipe::new(
                slot_id,
                DeviceContextIndex::default(),
                doorbell,
                allocator,
            )?,
            doorbell: Rc::clone(doorbell),
            phase: InitializePhase::NotPrepared,
            data_buff: [0; DATA_BUFF],
            endpoint_configs: Vec::new(),
            setup_stage: None,
            hid_buff: [0; 1024],
            mouse: Mouse::new(),
            interface_num: 0,
            ring2: Self::allocate_transfer_ring(allocator)?,
        })
    }
    fn allocate_transfer_ring(allocator: &mut impl MemoryAllocatable) -> PciResult<TransferRing> {
        let transfer_ring_addr = allocator.try_allocate_trb_ring(256)?;

        Ok(TransferRing::new(transfer_ring_addr, 256, true))
    }
    pub fn device_context_addr(&self) -> u64 {
        (&self.device_context.0 as *const Device32Byte) as u64
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
    fn set_enable_endpoint(&mut self, device_context_index: DeviceContextIndex) {
        self.input_context
            .0
            .control_mut()
            .set_add_context_flag(device_context_index.value());
    }

    fn notify(&self, dci: DeviceContextIndex) -> PciResult {
        self.doorbell
            .borrow_mut()
            .notify_at(self.slot_id as usize, dci.as_u8(), 0)
    }
}

fn make_setup_stage(setup_data: SetupStage, transfer_type: TransferType) -> SetupStage {
    let mut setup = SetupStage::new();
    setup.set_request_type(setup_data.request_type());
    setup.set_request(setup_data.request());
    setup.set_value(setup_data.value());
    setup.set_index(setup_data.index());
    setup.set_length(setup_data.length());
    setup.set_transfer_type(transfer_type);
    setup
}

fn make_data_stage(data_buff_addr: u64, length: u32, dir_in: bool) -> DataStage {
    // // Standard
    let mut data_stage = DataStage::new();

    data_stage.set_data_buffer_pointer(data_buff_addr);
    data_stage.set_trb_transfer_length(length);
    data_stage.set_td_size(0);
    if dir_in {
        data_stage.set_direction(In);
    } else {
        data_stage.set_direction(Out);
    }

    data_stage
}

fn max_packet_size(port_speed: u8) -> u16 {
    println!("Port Speed = {}", port_speed);
    match port_speed {
        3 => 64,
        4 => 512,
        _ => 8,
    }
}
