use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::event::{CommandCompletion, PortStatusChange, TransferEvent};

use registers::traits::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;
use registers::traits::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use registers::traits::registers_operation::RegistersOperation;
use registers::traits::usb_command_register_accessible::UsbCommandRegisterAccessible;
use transfer::event::event_ring::EventRing;

use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::collectable::single_device_collector::SingleDeviceCollector;
use crate::xhc::device_manager::collectable::DeviceCollectable;
use crate::xhc::device_manager::DeviceManager;
use crate::xhc::registers::traits::capability_registers_accessible::CapabilityRegistersAccessible;
use crate::xhc::registers::traits::config_register_accessible::ConfigRegisterAccessible;
use crate::xhc::registers::traits::device_context_bae_address_array_pointer_accessible::setup_device_manager;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::interrupter_set_register_accessible::setup_event_ring;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::registers::traits::usb_command_register_accessible::setup_command_ring;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event::event_trb::EventTrb;
use crate::xhc::transfer::event::target_event::TargetEvent;
use crate::xhc::transfer::trb_raw_data::TrbRawData;

pub mod allocator;
pub mod device_manager;
pub mod registers;
pub mod transfer;

pub struct XhcController<Register, Collectable, Memory>
where
    Register: RegistersOperation
        + InterrupterSetRegisterAccessible
        + PortRegistersAccessible
        + DoorbellRegistersAccessible
        + 'static,
    Collectable: DeviceCollectable<Register, Memory>,
    Memory: MemoryAllocatable,
{
    registers: Rc<RefCell<Register>>,
    event_ring: EventRing<Register>,
    command_ring: CommandRing<Register>,
    device_manager: DeviceManager<Register, Collectable, Memory>,
    allocator: Rc<RefCell<Memory>>,
}

impl<Register, Memory> XhcController<Register, SingleDeviceCollector<Register, Memory>, Memory>
where
    Register: RegistersOperation
        + CapabilityRegistersAccessible
        + InterrupterSetRegisterAccessible
        + UsbCommandRegisterAccessible
        + DoorbellRegistersAccessible
        + PortRegistersAccessible
        + ConfigRegisterAccessible
        + DeviceContextBaseAddressArrayPointerAccessible
        + 'static,
    Memory: MemoryAllocatable,
{
    pub fn new(
        registers: Register,
        mut allocator: Memory,
        mouse_driver_factory: MouseDriverFactory,
    ) -> PciResult<Self> {
        let mut registers = Rc::new(RefCell::new(registers));

        registers.borrow_mut().reset()?;

        registers.borrow_mut().write_max_device_slots_enabled(8)?;

        let scratchpad_buffers_len = registers.borrow().read_max_scratchpad_buffers_len();
        let device_manager = setup_device_manager(
            &mut registers,
            8,
            scratchpad_buffers_len,
            &mut allocator,
            mouse_driver_factory,
        )?;

        let command_ring = setup_command_ring(&mut registers, 32, &mut allocator)?;

        let (_, event_ring) = setup_event_ring(&mut registers, 1, 32, &mut allocator)?;

        registers.borrow_mut().run()?;

        Ok(Self {
            registers,
            event_ring,
            command_ring,
            device_manager,
            allocator: Rc::new(RefCell::new(allocator)),
        })
    }

    pub fn start_event_pooling(&mut self) -> PciResult {
        loop {
            self.check_event()?;
        }
    }

    pub fn check_event(&mut self) -> PciResult {
        if let Some(event_trb) = self.event_ring.read_event_trb() {
            self.on_event(event_trb)?;
        }

        Ok(())
    }

    fn on_event(&mut self, event_trb: EventTrb) -> PciResult {
        match event_trb {
            EventTrb::TransferEvent {
                transfer_event,
                target_event,
            } => self.on_transfer_event(transfer_event, target_event)?,
            EventTrb::CommandCompletionEvent(completion) => {
                self.process_completion_event(completion)?
            }
            EventTrb::PortStatusChangeEvent(port_status) => self.enable_slot(port_status)?,
            EventTrb::NotSupport { .. } => {}
        };

        self.event_ring.next_dequeue_pointer()
    }

    fn on_transfer_event(
        &mut self,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult {
        let is_init = self.device_manager.process_transfer_event(
            transfer_event.slot_id(),
            transfer_event,
            target_event,
        )?;

        if is_init {
            self.command_ring.push_configure_endpoint(
                self.device_manager
                    .device_slot_at(transfer_event.slot_id())
                    .unwrap()
                    .input_context_addr(),
                transfer_event.slot_id(),
            )?;
        }
        Ok(())
    }

    fn process_completion_event(&mut self, completion: CommandCompletion) -> PciResult {
        match TrbRawData::from_addr(completion.command_trb_pointer())
            .template()
            .trb_type()
        {
            9 => self.address_device(completion), // Enable Slot
            11 => self.init_device(completion),   // Address Device
            12 => self.device_manager.configure_endpoint(completion.slot_id()),
            _ => Ok(()),
        }
    }
    fn init_device(&mut self, completion: CommandCompletion) -> PciResult {
        self.device_manager
            .start_initialize_at(completion.slot_id())?;

        Ok(())
    }
    fn address_device(&mut self, completion: CommandCompletion) -> PciResult {
        let input_context_addr = self
            .device_manager
            .address_device(completion.slot_id(), &self.allocator)?;
        self.command_ring
            .push_address_command(input_context_addr, completion.slot_id())
    }

    fn enable_slot(&mut self, port_status: PortStatusChange) -> PciResult {
        let port_id = port_status.port_id();
        self.registers
            .borrow_mut()
            .clear_port_reset_change_at(port_id)?;
        self.command_ring.push_enable_slot()?;
        self.device_manager.set_addressing_port_id(port_id);
        Ok(())
    }
}

pub(crate) fn bit_mask_zeros_lower_for(bits: u32, target_value: usize) -> usize {
    let mask = !0 >> (usize::BITS - bits);
    // 下位5Bitsは予約領域
    target_value & !mask
}
