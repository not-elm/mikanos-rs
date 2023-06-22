use kernel_lib::error::KernelResult;
use kernel_lib::interrupt::gate_type::GateType;
use kernel_lib::interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute;
use kernel_lib::interrupt::interrupt_vector::InterruptVector;
use kernel_lib::interrupt::IDT;

use crate::interrupt::overflow::interrupt_overflow;
use crate::interrupt::page_fault::page_fault_handler;
use crate::interrupt::timer::interrupt_timer_handler;

use self::xhci::interrupt_xhci_handler;

mod overflow;
mod page_fault;
pub mod timer;
pub mod xhci;


pub fn init_idt() -> KernelResult {
    unsafe {
        let type_attribute = InterruptDescriptorAttribute::new()
            .with_gate_type(GateType::InterruptGate)
            .with_present(true);

        IDT[InterruptVector::Overflow].set_handler(interrupt_overflow, type_attribute)?;
        IDT[InterruptVector::PageFault]
            .set_page_fault_handler(page_fault_handler, type_attribute)?;
        IDT[InterruptVector::Xhci].set_handler(interrupt_xhci_handler, type_attribute)?;
        IDT[InterruptVector::ApicTimer].set_handler(interrupt_timer_handler, type_attribute)?;
        IDT.load();
    }

    Ok(())
}
