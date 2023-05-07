use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B2, B3, B4};

#[bitfield]
#[derive(Debug, Clone)]
pub struct EndpointDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub endpoint_address: EndpointAddress,
    pub attributes: Attributes,
    pub max_packet_size: u16,
    pub interval: u8,
}


#[bitfield]
#[derive(Debug, BitfieldSpecifier)]
pub struct EndpointAddress {
    pub number: B4,
    #[skip]
    reserve: B3,
    pub dir_in: bool,
}


#[bitfield]
#[derive(Debug, BitfieldSpecifier)]
pub struct Attributes {
    pub transfer_type: B2,

    pub sync_type: B2,

    pub usage_type: B2,

    #[skip]
    reserve: B2,
}

pub const ENDPOINT_DESCRIPTOR_TYPE: u8 = 5;
