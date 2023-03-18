use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::error::OperationReason::MustBeCommandRingStopped;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use crate::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;

/// CA
///
/// このレジスタに1を書き込むと現在のコマンドの実行を直ちに終了し、コマンドリングの動作を停止します。
/// その後、CommandRingStoppedを表すCompletionCodeを生成します。
///
/// Note: 読み込むときは常に0になります。
///
/// Note: CommandRingRunning(CRR)が1の場合、書き込みは無視されます。
#[derive(VolatileBits)]
#[volatile_type(u8)]
#[bits(1)]
#[offset(2)]
pub struct CommandAbort(usize);

impl CommandAbort {
    pub fn new(offset: CommandRingControlRegisterOffset) -> PciResult<Self> {
        let s = Self::new_uncheck(offset.offset());
        if s.read_flag_volatile() {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: false },
            ))
        } else {
            Ok(s)
        }
    }

    pub fn abort_command(crr: &CommandRingRunning) -> PciResult {
        if crr.read_flag_volatile() {
            return Err(PciError::FailedOperateToRegister(MustBeCommandRingStopped));
        }

        crr.write_flag_volatile(true);
        Ok(())
    }
}
