use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{OperationReason, PciError, PciResult};
use crate::xhci::registers::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use crate::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;

/// CS
///
/// このレジスタに1を書き込むと現在のコマンド実行後にコマンドリングの動作を停止させます。
///
/// Note: 読み込むときは常に0になります。
///
/// Note: CommandRingRunning(CRR)が1の場合、書き込みは無視されます。
#[derive(VolatileBits)]
#[volatile_type(u8)]
#[bits(1)]
#[offset_bit(1)]
pub struct CommandStop(usize, PhantomData<CommandRingControlRegisterOffset>);

impl CommandStop {
    pub fn stop_command(crr: &CommandRingRunning) -> PciResult {
        if crr.read_flag_volatile() {
            return Err(PciError::FailedOperateToRegister(
                OperationReason::MustBeCommandRingStopped,
            ));
        }

        crr.write_flag_volatile(true);
        Ok(())
    }
}
