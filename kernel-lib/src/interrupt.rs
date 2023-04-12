use crate::interrupt::interrupt_descriptor::{make_idt_attr, set_idt_entry, InterruptDescriptor};
use crate::segment::asm::read_cr;

pub mod asm;
pub mod interrupt_descriptor;
pub mod interrupt_descriptor_table_pointer;
pub mod interrupt_queue_waiter;

const IDT_SIZE: usize = 256;
static mut IDT: [InterruptDescriptor; IDT_SIZE] = [InterruptDescriptor::new(); IDT_SIZE];


pub unsafe fn init_idt() {
    set_idt_entry(
        &mut IDT[0x40],
        make_idt_attr(14, 0, true, 0),
        addr,
        read_cr(),
    );


    LoadIDT(
        (core::mem::size_of::<InterruptDescriptor>() * (IDT.len() - 1)) as u16,
        IDT.as_ptr() as u64,
    );
}
