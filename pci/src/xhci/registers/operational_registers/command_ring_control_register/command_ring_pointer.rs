use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;

/// CommandRingPointer
///
/// コマンドリングのでキューポインタのアドレスを設定するためのフィールドです。
///
/// Note: 読み込むときは常に0になります。
///
/// Note: CommandRingRunning(CRR)が1の場合、書き込みは無視されます。
///
/// Note: CommandRingは64Byteにアラインされてる必要があるため、
/// リングポインタの下位6ビットは常に0にする必要があります。
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[bits(58)]
#[offset(6)]
pub struct CommandRingPointer(usize);

impl CommandRingPointer {
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
}
