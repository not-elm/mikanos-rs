use crate::error::{OperationReason, PciError, PciResult};
use crate::error::OperationReason::FailedAllocate;
use crate::VolatileAccessible;
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
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

/// Address: OperationalRegistersOffset + 0x18
///
/// XhciPdfPageNo: 401
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

/// Address: OperationalRegistersOffset + 0x18
///
/// XhciPdfPageNo: 401
#[derive(Debug)]
pub struct CommandRingControlRegister {
    /// Offset: 0
    pub rcs: RingCycleState,
    /// Offset: 1 Bit
    pub cs: CommandStop,
    /// Offset: 2 Bit
    pub ca: CommandAbort,
    /// Offset: 3 Bit
    pub crr: CommandRingRunning,
    /// Offset: 6 Bit
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

    pub fn setup_command_ring(&self, allocator: &mut impl MemoryAllocatable) -> PciResult {
        let _command_ring_ptr_addr = unsafe { allocate_command_ring(self, allocator) }?;
        // TODO アドレスからCommandRingにキャスト
        Ok(())
    }
}

unsafe fn allocate_command_ring(
    crcr: &CommandRingControlRegister,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<usize> {
    const TRB_SIZE: usize = 128;

    let alloc_size = TRB_SIZE * 32;
    let command_ring_ptr_addr = allocator
        .allocate_with_align(alloc_size, 64, 64 * 1024)
        .ok_or(PciError::FailedOperateToRegister(FailedAllocate))?
        .address()?;

    register_command_ring(crcr, command_ring_ptr_addr as u64)?;
    Ok(command_ring_ptr_addr)
}

fn register_command_ring(crcr: &CommandRingControlRegister, command_ring_addr: u64) -> PciResult {
    if crcr.cs.read_flag_volatile() || crcr.ca.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::MustBeCommandRingStopped,
        ));
    }
    crcr.rcs.write_flag_volatile(true);
    crcr.command_ring_pointer
        .set_command_ring_addr(command_ring_addr);
    Ok(())
}
