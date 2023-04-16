use modular_bitfield::{
    bitfield,
    specifiers::{B3, B5},
};
use x86_64::instructions::segmentation::Segment;
use x86_64::registers::segmentation::CS;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::error::KernelResult;
use crate::interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute;

pub type PageFaultHandler =
    extern "x86-interrupt" fn(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode);

pub type InterruptHandler = extern "x86-interrupt" fn(stack_frame: InterruptStackFrame);

#[bitfield(bits = 128)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptDescriptor {
    pub offset_low: u16,
    pub segment_selector: u16,
    pub interrupt_stack_table_offset: B5,
    #[skip]
    __: B3,
    pub type_attributes: InterruptDescriptorAttribute,
    pub offset_middle: u16,
    pub offset_high: u32,

    #[skip]
    __: u32,
}


impl InterruptDescriptor {
    pub fn set_page_fault_handler(
        &mut self,
        handler: PageFaultHandler,
        type_attributes: InterruptDescriptorAttribute,
    ) -> KernelResult {
        let offset = handler as usize;
        self.set_type_attributes(type_attributes);
        self.set_offset_low(offset as u16);
        self.set_offset_middle((offset >> 16) as u16);
        self.set_offset_high((offset >> 32) as u32);
        self.set_segment_selector(CS::get_reg().0);

        Ok(())
    }
    pub fn set_handler(
        &mut self,
        handler: InterruptHandler,
        type_attributes: InterruptDescriptorAttribute,
    ) -> KernelResult {
        let offset = handler as usize;
        self.set_type_attributes(type_attributes);
        self.set_offset_low(offset as u16);
        self.set_offset_middle((offset >> 16) as u16);
        self.set_offset_high((offset >> 32) as u32);
        self.set_segment_selector(CS::get_reg().0);

        Ok(())
    }
}
