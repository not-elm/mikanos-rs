use core::marker::PhantomData;

use crate::xhc::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;
use macros::VolatileBits;

/// HCH
///
/// Default = 1
///
/// RunStopが1の間、0になります。
#[derive(VolatileBits)]
#[bits(1)]
pub struct HostControllerHalted(usize, PhantomData<UsbStatusRegisterOffset>);

impl HostControllerHalted {
    pub fn until_not_halted(&self) {
        while self.read_flag_volatile() {}
    }

    pub fn until_halted(&self) {
        while !self.read_flag_volatile() {}
    }
}
