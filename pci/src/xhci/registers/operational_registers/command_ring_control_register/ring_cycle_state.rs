use macros::VolatileBits;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;

/// RCS
/// TODO 調査中
/// Note: 読み込むときは常に0になります。
/// Note: CommandRingRunning(CRR)が1の場合、書き込みは無視されます。
#[derive(VolatileBits)]
#[volatile_type(u8)]
#[bits(1)]
pub struct RingCycleState(usize);

impl RingCycleState {
    pub fn new(offset: CommandRingControlRegisterOffset) -> PciResult<Self> {
        let s = Self::new_uncheck(offset.offset());
        if s.read_flag_volatile() {
            Err(PciError::InvalidRingCycleState)
        } else {
            Ok(s)
        }
    }
}
