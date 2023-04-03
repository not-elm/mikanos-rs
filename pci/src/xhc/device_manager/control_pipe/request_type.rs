use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct RequestType {
    #[bits(5)]
    pub recipient: u8,
    #[bits(2)]
    pub ty: u8,
    pub direction: bool,
}

impl RequestType {
    pub fn raw(&self) -> u8 {
        self.0
    }
}
