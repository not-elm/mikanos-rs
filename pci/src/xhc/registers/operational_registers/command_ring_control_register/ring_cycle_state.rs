use crate::xhc::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;
use core::marker::PhantomData;
use macros::VolatileBits;

/// RCS
/// TODO 調査中
///
/// Note: 読み込むときは常に0になります。
///
/// Note: CommandRingRunning(CRR)が1の場合、書き込みは無視されます。
#[derive(VolatileBits)]
#[volatile_type(u8)]
#[bits(1)]
pub struct RingCycleState(usize, PhantomData<CommandRingControlRegisterOffset>);
