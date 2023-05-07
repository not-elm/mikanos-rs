use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B2, B5};

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone)]
pub struct RequestType {
    pub recipient: B5,
    pub ty: B2,
    pub direction: bool,
}

impl RequestType {
    pub fn raw(&self) -> u8 {
        self.clone().into_bytes()[0]
    }
}
