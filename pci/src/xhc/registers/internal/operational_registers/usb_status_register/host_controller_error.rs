use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhc::registers::internal::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

/// HCH
///
/// Default = 1
///
/// RunStopが1の間、0になります。
#[derive(VolatileBits)]
#[bits(1)]
#[offset_bit(12)]
pub struct HostControllerError(usize, PhantomData<UsbStatusRegisterOffset>);
