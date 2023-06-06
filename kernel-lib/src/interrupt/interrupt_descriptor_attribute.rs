use modular_bitfield::bitfield;
use modular_bitfield::prelude::B2;

use crate::interrupt::gate_type::GateType;

#[bitfield]
#[repr(u8)]
#[derive(Debug, Copy, Clone, BitfieldSpecifier)]
pub struct InterruptDescriptorAttribute {
    pub gate_type: GateType,
    #[allow(non_snake_case)]
    #[skip]
    __: bool,
    pub descriptor_privilege_level: B2,
    pub present: bool,
}
