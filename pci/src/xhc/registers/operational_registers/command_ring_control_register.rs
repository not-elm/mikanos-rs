use crate::error::{OperationReason, PciError, PciResult};
use crate::VolatileAccessible;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::operational_registers::command_ring_control_register::command_abort::CommandAbort;
use crate::xhc::registers::operational_registers::command_ring_control_register::command_ring_pointer::CommandRingPointer;
use crate::xhc::registers::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use crate::xhc::registers::operational_registers::command_ring_control_register::command_stop::CommandStop;
use crate::xhc::registers::operational_registers::command_ring_control_register::crcr_field::CrcrField;
use crate::xhc::registers::operational_registers::command_ring_control_register::ring_cycle_state::RingCycleState;
use crate::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

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
            rcs: RingCycleState::new(offset),
            cs: CommandStop::new(offset),
            ca: CommandAbort::new(offset),
            crr: CommandRingRunning::new(offset),
            command_ring_pointer: CommandRingPointer::new(offset),
        })
    }
    pub fn setup_command_ring(&self, allocator: &mut impl MemoryAllocatable) -> PciResult {
        let _command_ring_ptr_addr = unsafe { allocate_command_ring(self, allocator) }?;
        // TODO アドレスからCommandRingにキャスト
        Ok(())
    }
    pub fn ring_cycle_state(&self) -> &RingCycleState {
        &self.rcs
    }

    pub fn command_stop(&self) -> &CommandStop {
        &self.cs
    }

    pub fn command_abort(&self) -> &CommandAbort {
        &self.ca
    }

    pub fn command_ring_running(&self) -> &CommandRingRunning {
        &self.crr
    }

    pub fn command_ring_pointer(&self) -> &CommandRingPointer {
        &self.command_ring_pointer
    }

    pub fn register_command_ring(&self, command_ring_address: u64) -> PciResult {
        register_command_ring(self, command_ring_address)
    }
}

unsafe fn allocate_command_ring(
    crcr: &CommandRingControlRegister,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<u64> {
    const TRB_SIZE: usize = 128;

    let alloc_size = TRB_SIZE * 32;
    let command_ring_ptr_addr = allocator
        .try_allocate_with_align(alloc_size, 64, 64 * 1024)?
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

    crcr.cs.write_flag_volatile(false);
    crcr.ca.write_flag_volatile(false);

    crcr.command_ring_pointer
        .update_command_ring_addr(command_ring_addr)?;
    crcr.rcs.write_flag_volatile(true);

    Ok(())
}
