use core::marker::PhantomData;

use macros::VolatileBits;

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
pub struct CommandRingPointer(usize, PhantomData<CommandRingControlRegisterOffset>);

impl CommandRingPointer {
    pub fn set_command_ring_addr(&self, addr: u64) {
        self.write_volatile(addr >> 6);
    }
}
