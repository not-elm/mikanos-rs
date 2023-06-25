use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Debug;

use xhci::ring::trb::event::{CommandCompletion, PortStatusChange, TransferEvent};

use kernel_lib::serial_println;
use transfer::event::event_ring::EventRing;

use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::mouse::driver::MouseDriver;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::DeviceManager;
use crate::xhc::registers::traits::device_context_bae_address_array_pointer_accessible::setup_device_manager;
use crate::xhc::registers::traits::interrupter::setup_event_ring;
use crate::xhc::registers::traits::usb_command::setup_command_ring;
use crate::xhc::registers::XhcRegisters;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event::event_trb::EventTrb;
use crate::xhc::transfer::event::target_event::TargetEvent;
use crate::xhc::transfer::trb_raw_data::TrbRawData;
use crate::xhc::waiting_ports::WaitingPorts;

pub mod allocator;
pub mod device_manager;
pub mod registers;
pub mod transfer;
mod waiting_ports;

pub struct XhcController<Register, Memory> {
    registers: Rc<RefCell<Register>>,
    event_ring: EventRing<Register>,
    command_ring: CommandRing<Register>,
    waiting_ports: WaitingPorts,
    device_manager: DeviceManager<Register, Memory>,
    allocator: Rc<RefCell<Memory>>,
}


impl<Register, Memory> XhcController<Register, Memory>
where
    Register: XhcRegisters + 'static + Debug,
    Memory: MemoryAllocatable,
{
    pub fn new(
        registers: Register,
        mut allocator: Memory,
        mouse: MouseDriver,
        keyboard: KeyboardDriver,
    ) -> PciResult<Self> {
        let mut registers = Rc::new(RefCell::new(registers));

        registers
            .borrow_mut()
            .reset()?;

        registers
            .borrow_mut()
            .write_max_device_slots_enabled(8)?;

        let scratchpad_buffers_len = registers
            .borrow()
            .read_max_scratchpad_buffers_len();

        let device_manager = setup_device_manager(
            &mut registers,
            8,
            scratchpad_buffers_len,
            &mut allocator,
            mouse,
            keyboard,
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
            waiting_ports: WaitingPorts::default(),
        })
    }


    pub fn reset_port(&mut self) -> PciResult {
        let connect_ports = self
            .registers
            .borrow()
            .connecting_ports();

        if connect_ports.is_empty() {
            return Ok(());
        }

        self.registers
            .borrow_mut()
            .reset_port_at(connect_ports[0])?;

        // Since it is necessary to process one by one up to the address command,
        // other ports are saved in the reserved area.
        for port_id in connect_ports
            .into_iter()
            .skip(1)
        {
            self.waiting_ports
                .push(port_id);
        }

        Ok(())
    }


    pub fn start_event_pooling(&mut self) -> ! {
        loop {
            let _ = self
                .process_event()
                .map(|p| p.unwrap());
        }
    }


    pub fn process_all_events(&mut self) {
        while self.event_ring.has_front() {
            serial_println!("{:?}", self.registers.borrow());
            self.process_event();
        }

        self.registers
            .borrow_mut()
            .set_counter_at(0, 100);
        self.registers
            .borrow_mut()
            .write_interrupter_pending_at(0, true);

        serial_println!("{:?}", self.registers.borrow());
    }


    pub fn process_event(&mut self) -> Option<PciResult> {
        if let Some(event_trb) = self
            .event_ring
            .read_event_trb()
        {
            return Some(self.on_event(event_trb));
        }

        None
    }


    fn on_event(&mut self, event_trb: EventTrb) -> PciResult {
        serial_println!("{:?}", event_trb);

        match event_trb {
            EventTrb::TransferEvent {
                transfer_event,
                target_event,
            } => {
                self.on_transfer_event(transfer_event, target_event)?;
            }
            EventTrb::CommandCompletionEvent(completion) => {
                self.process_completion_event(completion)?
            }
            EventTrb::PortStatusChangeEvent(port_status) => {
                self.on_port_status_change(port_status)?
            }
            EventTrb::NotSupport { .. } => {}
        };

        self.event_ring
            .next_dequeue_pointer()
    }


    fn on_transfer_event(
        &mut self,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult {
        let slot_id = transfer_event.slot_id();

        let is_init = self
            .device_manager
            .process_transfer_event(slot_id, transfer_event, target_event)?;

        if is_init {
            self.configure_endpoint(slot_id)?;
        }

        Ok(())
    }


    fn configure_endpoint(&mut self, slot_id: u8) -> PciResult {
        let input_context_addr = self
            .device_manager
            .device_slot_at(slot_id)
            .unwrap()
            .input_context_addr();

        self.command_ring
            .push_configure_endpoint(input_context_addr, slot_id)
    }


    fn process_completion_event(&mut self, completion: CommandCompletion) -> PciResult {
        match TrbRawData::from_addr(completion.command_trb_pointer())
            .template()
            .trb_type()
        {
            // Enable Slot
            9 => self.address_device(completion),
            // Address Device
            11 => self.init_device(completion),
            // Configure Endpoint
            12 => self
                .device_manager
                .configure_endpoint(completion.slot_id()),
            _ => Ok(()),
        }
    }


    fn init_device(&mut self, completion: CommandCompletion) -> PciResult {
        self.reset_waiting_port_if_need()?;

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


    fn on_port_status_change(&mut self, port_status: PortStatusChange) -> PciResult {
        let port_id = port_status.port_id();
        if self
            .device_manager
            .is_addressing_port(port_id)
        {
            self.enable_slot(port_id)?;
        } else {
            self.waiting_ports
                .push(port_id);
        }

        Ok(())
    }


    fn enable_slot(&mut self, port_id: u8) -> PciResult {
        self.registers
            .borrow_mut()
            .clear_port_reset_change_at(port_id)?;

        self.device_manager
            .set_addressing_port_id(port_id);

        self.command_ring
            .push_enable_slot()
    }


    fn reset_waiting_port_if_need(&mut self) -> PciResult {
        if let Some(port_id) = self.waiting_ports.pop() {
            self.registers
                .borrow_mut()
                .reset_port_at(port_id)?;
        }

        Ok(())
    }
}


#[allow(unused)]
pub(crate) fn bit_mask_zeros_lower_for(bits: u32, target_value: usize) -> usize {
    let mask = !0 >> (usize::BITS - bits);
    // 下位5Bitsは予約領域
    target_value & !mask
}
