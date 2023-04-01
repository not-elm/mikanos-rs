use bitfield_struct::bitfield;

#[repr(packed)]
#[derive(Debug)]
pub struct EndpointDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub endpoint_address: EndpointAddress,
    pub attributes: Attributes,
    pub max_packet_size: u16,
    pub interval: u8,
}

#[bitfield(u8)]
pub struct EndpointAddress {
    #[bits(4)]
    pub number: u8,
    #[bits(3)]
    _reserve: u8,
    pub dir_in: bool,
}

#[bitfield(u8)]
pub struct Attributes {
    #[bits(2)]
    pub transfer_type: u8,
    #[bits(2)]
    pub sync_type: u8,
    #[bits(2)]
    pub usage_type: u8,
    #[bits(2)]
    _reserve: u8,
}

pub const ENDPOINT_DESCRIPTOR_TYPE: u8 = 5;
