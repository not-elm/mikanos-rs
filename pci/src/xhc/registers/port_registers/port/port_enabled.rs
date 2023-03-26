use crate::xhc::registers::port_registers::port::PortRegisterAddr;
use core::marker::PhantomData;
use macros::VolatileBits;

#[derive(VolatileBits)]
#[bits(1)]
#[offset_bit(1)]
pub struct PortEnabled(usize, PhantomData<PortRegisterAddr>);
