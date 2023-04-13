use modular_bitfield::{
    bitfield,
    specifiers::{B3, B5},
};
use x86_64::structures::idt::InterruptStackFrame;

use crate::{
    interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute,
    segment::asm::read_code_segment,
};
use crate::error::{KernelError, KernelResult};

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
    pub fn set_handler(
        &mut self,
        handler: InterruptHandler,
        type_attributes: InterruptDescriptorAttribute,
    ) -> KernelResult {
        let offset = u64::try_from(handler as usize).map_err(|_| KernelError::FailedCast)?;
        self.set_type_attributes(type_attributes);
        self.set_offset_low((offset & 0xffff) as u16);
        self.set_offset_middle(((offset >> 16) & 0xffff) as u16);
        self.set_offset_high((offset >> 32) as u32);
        self.set_segment_selector(read_code_segment());

        Ok(())
    }
}
