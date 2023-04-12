use crate::interrupt::interrupt_descriptor::InterruptDescriptor;

pub mod asm;
pub mod interrupt_descriptor;
pub mod interrupt_queue_waiter;


const IDT_SIZE: usize = 256;
static mut IDT: [InterruptDescriptor; IDT_SIZE] = [InterruptDescriptor::new(); IDT_SIZE];
