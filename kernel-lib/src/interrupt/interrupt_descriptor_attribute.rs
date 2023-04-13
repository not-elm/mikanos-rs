use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B2, B3, B4, B5};

#[bitfield(bits = 16)]
#[derive(Debug, Copy, Clone, BitfieldSpecifier)]
pub struct InterruptDescriptorAttribute {
    pub interrupt_stack_table: B3,
    #[skip]
    __: B5,
    pub descriptor_type: B4,
    #[skip]
    __: bool,
    pub descriptor_privilege_level: B2,
    pub present: bool,
}
