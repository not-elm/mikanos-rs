use crate::xhci::transfer::request_block::trb_template::TrbAddr;
use core::marker::PhantomData;
use macros::VolatileBits;

///
///
#[derive(VolatileBits)]
#[bits(1)]
#[add_addr_bytes(0x0C)]
pub struct CycleBit(usize, PhantomData<TrbAddr>);
