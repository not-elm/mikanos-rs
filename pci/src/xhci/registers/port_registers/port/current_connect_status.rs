use crate::xhci::registers::port_registers::port::PortRegisterAddr;
use core::marker::PhantomData;
use macros::VolatileBits;

#[derive(VolatileBits)]
#[bits(1)]
pub struct CurrentConnectStatus(usize, PhantomData<PortRegisterAddr>);
