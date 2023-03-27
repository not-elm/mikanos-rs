use crate::xhc::registers::internal::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;
use core::marker::PhantomData;
use macros::VolatileBits;

/// IP
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
/// このフラグが1の時、割り込み要求が発生し、保留中となっていることを表します。
///
/// 0のときは割り込み要求がありません。
///
///
/// [Xhci Document] : 425 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[bits(1)]
pub struct InterruptPending(usize, PhantomData<InterrupterRegisterSetOffset>);
