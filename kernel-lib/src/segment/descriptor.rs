use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B2, B4};

/// Segment Descriptor
#[bitfield(bits = 64)]
pub struct SegmentDescriptor {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub descriptor_type: B4,
    pub system_segment: bool,
    pub descriptor_privilege_level: B2,
    pub preset: bool,
    pub limit_high: B4,
    pub available: bool,
    pub long_mode: bool,
    pub default_operation_size: bool,
    pub granularity: bool,
    pub base_high: u8,
}
