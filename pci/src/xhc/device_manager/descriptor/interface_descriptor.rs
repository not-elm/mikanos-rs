#[derive(Debug)]
#[repr(packed)]
pub struct InterfaceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub num_endpoints: u8,
    pub interface_class: u8,
    pub interface_sub_class: u8,
    pub interface_protocol: u8,
    pub interface_id: u8,
}

pub const INTERFACE_DESCRIPTOR_TYPE: u8 = 4;

impl InterfaceDescriptor {
    pub fn is_mouse(&self) -> bool {
        self.interface_class == 3 && self.interface_sub_class == 1 && self.interface_protocol == 2
    }
}
