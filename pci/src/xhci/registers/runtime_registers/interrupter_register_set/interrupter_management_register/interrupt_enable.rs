use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;

/// IE
///
/// # Offset
///
/// 0
///
/// # Size
///
/// 1 Bit
///
/// # Default
///
/// 0
///
/// # Description
///
/// このフラグが1の時、割り込み要求を受け入れる状態になります。
///
/// 自身とIPが1の場合、InterruptModerationCounterが0に達したときに割り込みが発生する必要があります。
///
/// [Xhci Document] : 425 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[bits(1)]
#[offset_bit(1)]
pub struct InterruptEnable(usize, PhantomData<InterrupterRegisterSetOffset>);
