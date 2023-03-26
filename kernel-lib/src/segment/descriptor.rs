use bitfield_struct::bitfield;

/// Segment Descriptor
#[bitfield(u64)]
pub struct DescriptorA {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    #[bits(4)]
    pub descriptor_type: u8,
    pub system_segment: bool,
    #[bits(2)]
    pub descriptor_privilege_level: usize,
    pub preset: bool,
    #[bits(4)]
    pub limit_high: u8,
    pub available: bool,
    pub long_mode: bool,
    pub default_operation_size: bool,
    pub granularity: bool,
    pub base_high: u8,
}
