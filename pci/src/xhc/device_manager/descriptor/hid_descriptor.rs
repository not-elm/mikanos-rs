use modular_bitfield::bitfield;

#[bitfield(bits = 72)]
#[derive(Debug)]
pub struct HidDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub hid_release: u16,
    pub country_code: u8,
    pub num_descriptors: u8,
    pub class_descriptor: ClassDescriptor,
}

#[bitfield(bits = 24)]
#[derive(Debug, BitfieldSpecifier)]
pub struct ClassDescriptor {
    pub descriptor_type: u8,

    pub descriptor_length: u16,
}

pub(crate) const HID_DESCRIPTOR_TYPE: u8 = 33;
