use crate::interrupt::interrupt_descriptor_table::InterruptDescriptorTable;

pub mod asm;
pub mod gate_type;
mod idt_descriptor;
pub mod interrupt_descriptor;
pub mod interrupt_descriptor_attribute;
pub mod interrupt_descriptor_table;
pub mod interrupt_message;
pub mod interrupt_vector;


pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
