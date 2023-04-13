use core::ops::{Index, IndexMut};

use crate::interrupt::asm::load_idt;
use crate::interrupt::idt_descriptor::IdtDescriptor;
use crate::interrupt::interrupt_descriptor::InterruptDescriptor;

const IDT_SIZE: usize = 256;

pub struct InterruptDescriptorTable([InterruptDescriptor; IDT_SIZE]);

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self([InterruptDescriptor::new(); IDT_SIZE])
    }

    pub fn load(&self) {
        load_idt(&self.descriptor());
    }

    fn descriptor(&self) -> IdtDescriptor {
        IdtDescriptor::new(
            ((self.0.len() - 1) * core::mem::size_of::<InterruptDescriptor>()) as u16,
            self.0.as_ptr() as u64,
        )
    }
}


impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}


impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


impl Index<usize> for InterruptDescriptorTable {
    type Output = InterruptDescriptor;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
