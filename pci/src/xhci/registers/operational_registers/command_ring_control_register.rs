use crate::error::PciResult;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_abort::CommandAbort;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_ring_pointer::CommandRingPointer;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_stop::CommandStop;
use crate::xhci::registers::operational_registers::command_ring_control_register::crcr_field::CrcrField;
use crate::xhci::registers::operational_registers::command_ring_control_register::ring_cycle_state::RingCycleState;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

pub mod command_abort;
pub mod command_ring_pointer;
pub mod command_ring_running;
pub mod command_stop;
pub mod crcr_field;
pub mod ring_cycle_state;

#[derive(Debug)]
pub struct CommandRingControlRegister {
    pub rcs: RingCycleState,
    pub cs: CommandStop,
    pub ca: CommandAbort,
    pub crr: CommandRingRunning,
    pub command_ring_pointer: CommandRingPointer,
}

impl CommandRingControlRegister {
    pub fn new(offset: CommandRingControlRegisterOffset) -> PciResult<Self> {
        Ok(Self {
            rcs: RingCycleState::new_check_flag_false(offset)?,
            cs: CommandStop::new_check_flag_false(offset)?,
            ca: CommandAbort::new_check_flag_false(offset)?,
            crr: CommandRingRunning::new(offset),
            command_ring_pointer: CommandRingPointer::new(offset),
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CommandRingControlRegisterOffset(usize);

impl CommandRingControlRegisterOffset {
    pub fn new(offset: OperationalRegistersOffset) -> Self {
        Self(offset.offset() + 0x18)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
