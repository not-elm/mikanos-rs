#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ConfigurationDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub total_length: u16,
    pub num_interfaces: u8,
    pub configuration_value: u8,
    pub configuration_id: u8,
    pub attributes: u8,
    pub max_power: u8,
}

pub(crate) const CONFIGURATION_DESCRIPTOR_TYPE: u8 = 2;
