use modular_bitfield::prelude::{B2, B3, B4, B5};
use modular_bitfield::{bitfield, BitfieldSpecifier};

#[bitfield(bits = 16)]
#[derive(Debug, Copy, Clone, BitfieldSpecifier)]
pub struct InterruptDescriptorAttribute {
    pub interrupt_stack_table: B3,
    #[allow(non_snake_case)]
    _reserve: B5,
    pub descriptor_type: B4,
    #[allow(non_snake_case)]
    _reserve2: bool,
    pub descriptor_privilege_level: B2,
    pub present: bool,
}


#[bitfield(bits = 128)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptDescriptor {
    pub offset_low: u16,
    pub segment_selector: u16,
    pub attr: InterruptDescriptorAttribute,
    pub offset_middle: u16,
    pub offset_high: u32,
    pub reserved: u32,
}

pub fn make_idt_attr(
    descriptor_type: u8,
    descriptor_privilege_level: u8,
    present: bool,
    interrupt_stack_table: u8,
) -> InterruptDescriptorAttribute {
    InterruptDescriptor::new()
        .with_descriptor_type(descriptor_type)
        .with_descriptor_privilege_level(descriptor_privilege_level)
        .with_present(present)
        .with_interrupt_stack_table(interrupt_stack_table)
}

pub fn set_idt_entry(
    desc: &mut InterruptDescriptor,
    attr: InterruptDescriptorAttribute,
    offset: u64,
    segment_selector: u16,
) {
    desc.set_attr(attr);
    desc.set_offset_low((offset & 0xffff) as u16);
    desc.set_offset_middle(((offset >> 16) & 0xffff) as u16);
    desc.set_offset_high((offset >> 32) as u32);
    desc.set_segment_selector(segment_selector);
}
