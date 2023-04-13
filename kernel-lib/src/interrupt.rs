use crate::interrupt::interrupt_descriptor_table::InterruptDescriptorTable;

pub mod asm;
mod idt_descriptor;
pub mod interrupt_descriptor;
pub mod interrupt_descriptor_attribute;
pub mod interrupt_descriptor_table;
pub mod interrupt_queue_waiter;
pub mod interrupt_vector;
pub mod gate_type;


pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
