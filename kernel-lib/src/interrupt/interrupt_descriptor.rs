use modular_bitfield::bitfield;

use crate::interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute;

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


impl InterruptDescriptor {
    pub fn entry(
        &mut self,
        attr: InterruptDescriptorAttribute,
        offset: u64,
        segment_selector: u16,
    ) {
        self.set_attr(attr);
        self.set_offset_low((offset & 0xffff) as u16);
        self.set_offset_middle(((offset >> 16) & 0xffff) as u16);
        self.set_offset_high((offset >> 32) as u32);
        self.set_segment_selector(segment_selector);
    }
}
