use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

/// CNR
///
/// Default = 1
///
/// RunStopが1の間、0になります。
#[derive(VolatileBits)]
#[bits(1)]
#[offset_bit(11)]
#[volatile_type(u32)]
pub struct ControllerNotReady(usize, PhantomData<UsbStatusRegisterOffset>);

impl ControllerNotReady {
    pub fn wait_until_ready(&self) {
        while self.read_flag_volatile() {}
    }
}
