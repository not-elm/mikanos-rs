use alloc::rc::Rc;
use core::cell::RefCell;
use core::ptr::null_mut;

use bitfield_struct::bitfield;
use xhci::context::{Device32Byte, EndpointType, Input32Byte, InputHandler};
use xhci::ring::trb::event::TransferEvent;
use xhci::ring::trb::transfer::Direction::{In, Out};
use xhci::ring::trb::transfer::{DataStage, SetupStage, StatusStage, TransferType};

use kernel_lib::{println, serial_println};

use crate::error::{PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::descriptor::configuration_descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptors,
};
use crate::xhc::device_manager::descriptor::device_descriptor::DeviceDescriptor;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::device_manager::endpoint_id::EndpointId;
use crate::xhc::device_manager::initialize_phase::InitializePhase;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::event_trb::TargetEvent;
use crate::xhc::transfer::transfer_ring::TransferRing;

const DATA_BUFF: usize = 256;
const DATA_BUFF_SIZE: usize = DATA_BUFF / 4;

#[repr(C, align(64))]
#[derive(Debug)]
pub struct Device<T>
where
    T: DoorbellRegistersAccessible,
{
    input_context: InputContext,
    slot_id: u8,
    device_context: DeviceContext,
    transfer_ring: [TransferRing; 31],
    phase: InitializePhase,
    data_buff: [u8; DATA_BUFF],
    num_configurations: u8,
    setup_stage: Option<SetupStage>,
    doorbell: Rc<RefCell<T>>,
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
        doorbell: &Rc<RefCell<T>>,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        let mut me = Self::new(slot_id, doorbell, allocator)?;

        me.set_enable_slot_context();
        me.set_enable_device_context(calc_device_context_index(0, false));

        me.init_slot_context(parent_hub_slot_id, port_speed);
        me.init_default_control_pipe(port_speed);

        Ok(me)
    }
    pub fn is_init(&self) -> bool {
        match self.phase {
            InitializePhase::Completed => true,
            _ => false,
        }
    }
    pub fn start_initialize(&mut self) -> PciResult {
        serial_println!("1 Start Initialize");
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
                Ok(())
            }
            InitializePhase::Completed => Ok(()),
            _ => Err(PciError::NullPointer),
        }
    }

    fn initialize_phase1(&mut self, buff: *mut u8, len: u16) -> PciResult {
        serial_println!(" ====== Initialize Phase1 ========= ");

        let device_descriptor = unsafe { *(buff as *const DeviceDescriptor) };
        self.phase = InitializePhase::Phase2;
        serial_println!("{:?}", device_descriptor);

        self.num_configurations = device_descriptor.num_configurations;
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
        serial_println!(" ====== Initialize Phase2 ========= ");
        let conf_desc = unsafe { *(buff as *const ConfigurationDescriptor) };
        self.phase = InitializePhase::Phase3;

        serial_println!("{:?}", conf_desc);
        let ds = ConfigurationDescriptors::new(buff, len as usize);
        {
            let mut interfaces = ds.skip_while(|d| d.interface().is_none());
            let interface = interfaces.next();
        }
        // serial_println!("3 Initialize Phase2");
        // self.phase = InitializePhase::Phase3;
        // let config =
        //     unsafe { *(data_stage.data_buffer_pointer() as *const ConfigurationDescriptor) };
        // serial_println!("ConfigurationDescriptor");
        // serial_println!("{:?}", config);
        // let mut status = StatusStage::new();
        //
        // let mut setup_data = SetupStage::new();
        // setup_data.set_request(9);
        // setup_data.set_value(config.configuration_value as u16);
        //
        // self.transfer_ring.push(setup_data.into_raw())?;
        // self.transfer_ring
        //     .push(make_data_stage(0, 0, false).into_raw())?;
        // self.transfer_ring.push(status.into_raw())

        self.set_configuration(
            EndpointId::from_endpoint_num(0, true),
            conf_desc.configuration_value as u16,
        )
    }
    fn get_descriptor(
        &mut self,
        ep_id: EndpointId,
        desc_type: u16,
        desc_index: u16,
        buff: *mut u8,
        len: u16,
    ) -> PciResult {
        let mut setup_data = SetupStage::new();
        setup_data.set_request_type(RequestType::new().with_direction(true).into());
        setup_data.set_request(6);
        setup_data.set_value(desc_type << 8 | desc_index);
        setup_data.set_index(0);
        setup_data.set_length(len);

        self.control_in(ep_id, setup_data, buff, len)
    }

    fn set_configuration(&mut self, ep_id: EndpointId, config_value: u16) -> PciResult {
        let mut setup_data = SetupStage::new();
        setup_data.set_request_type(RequestType::new().into());
        const CONFIGURATION: u8 = 9;
        setup_data.set_request(CONFIGURATION);
        setup_data.set_value(config_value);
        setup_data.set_index(0);
        setup_data.set_length(0);

        self.control_out(ep_id, setup_data, null_mut(), 0)
    }
    fn control_in(
        &mut self,
        ep_id: EndpointId,
        setup_data: SetupStage,
        buff: *mut u8,
        len: u16,
    ) -> PciResult {
        let mut status = StatusStage::new();
        let mut tr = self.transfer_ring[ep_id.value() - 1];
        serial_println!("ep_id={}", ep_id.value());

        if !buff.is_null() {
            let setup = make_setup_stage(setup_data, TransferType::In);
            self.transfer_ring[ep_id.value() - 1].push(setup.clone().into_raw())?;

            let mut data_stage = make_data_stage(buff as u64, len as u32, true);
            data_stage.set_interrupt_on_completion();
            self.transfer_ring[ep_id.value() - 1].push(data_stage.into_raw())?;

            self.transfer_ring[ep_id.value() - 1].push(status.into_raw())?;
            self.setup_stage = Some(setup);
        } else {
            let setup_stage = make_setup_stage(setup_data, TransferType::No);
            tr.push(setup_stage.clone().into_raw())?;

            status.set_direction();
            status.set_interrupt_on_completion();
            tr.push(status.into_raw())?;
            self.setup_stage = Some(setup_stage);
        }
        self.notify(DeviceContextIndex::from_endpoint_id(ep_id))
    }
    fn control_out(
        &mut self,
        ep_id: EndpointId,
        setup_data: SetupStage,
        buff: *mut u8,
        len: u16,
    ) -> PciResult {
        let mut status = StatusStage::new();
        status.set_direction();
        let mut tr = self.transfer_ring[ep_id.value() - 1];

        if !buff.is_null() {
            let setup = make_setup_stage(setup_data, TransferType::Out);
            tr.push(setup.clone().into_raw())?;

            let mut data_stage = make_data_stage(buff as u64, len as u32, false);
            data_stage.set_interrupt_on_completion();
            tr.push(data_stage.into_raw())?;

            status.set_direction();
            status.set_interrupt_on_completion();
            tr.push(status.into_raw())?;
            self.setup_stage = Some(setup);
        } else {
            let setup_stage = make_setup_stage(setup_data, TransferType::No);
            tr.push(setup_stage.clone().into_raw())?;

            status.set_interrupt_on_completion();
            tr.push(status.into_raw())?;
            self.setup_stage = Some(setup_stage);
        }

        self.notify(DeviceContextIndex::from_endpoint_id(ep_id))
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
        default_control_pipe.set_tr_dequeue_pointer(self.transfer_ring[0].base_address());
        default_control_pipe.set_dequeue_cycle_state();
        default_control_pipe.set_interval(0);
        default_control_pipe.set_max_primary_streams(0);
        default_control_pipe.set_mult(0);
        default_control_pipe.set_error_count(3);
    }
    fn new(
        slot_id: u8,
        doorbell: &Rc<RefCell<T>>,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        Ok(Self {
            slot_id,
            input_context: InputContext(Input32Byte::default()),
            device_context: DeviceContext(Device32Byte::default()),
            transfer_ring: [Self::allocate_transfer_ring(allocator)?; 31],
            phase: InitializePhase::NotPrepared,
            data_buff: [0; DATA_BUFF],
            num_configurations: 0,
            setup_stage: None,
            doorbell: Rc::clone(doorbell),
        })
    }
    fn allocate_transfer_ring(allocator: &mut impl MemoryAllocatable) -> PciResult<TransferRing> {
        let transfer_ring_addr = allocator.try_allocate_trb_ring(256)?;

        Ok(TransferRing::new(transfer_ring_addr, 256, true))
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

    fn notify(&mut self, dci: DeviceContextIndex) -> PciResult {
        self.doorbell
            .borrow_mut()
            .notify_at(self.slot_id as usize, dci.as_u8(), 0)
    }
}

fn make_setup_stage(setup_data: SetupStage, transfer_type: TransferType) -> SetupStage {
    // // Standard
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
