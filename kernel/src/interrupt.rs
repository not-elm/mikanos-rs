use x86_64::registers::read_rip;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use kernel_lib::error::KernelResult;
use kernel_lib::interrupt::gate_type::GateType;
use kernel_lib::interrupt::IDT;
use kernel_lib::interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute;
use kernel_lib::interrupt::interrupt_vector::InterruptVector;
use kernel_lib::serial_println;

use crate::interrupt::overflow::interrupt_overflow;
use crate::interrupt::timer::interrupt_timer_handler;
use crate::println;

use self::mouse::interrupt_mouse_handler;

pub mod interrupt_queue_waiter;
pub mod mouse;
mod overflow;
mod timer;

#[derive(Debug)]
pub enum InterruptMessage {
    Xhci,
    ApicTimer,
}


pub fn init_idt() -> KernelResult {
    unsafe {
        let type_attribute = InterruptDescriptorAttribute::new()
            .with_gate_type(GateType::InterruptGate)
            .with_present(true);

        IDT[InterruptVector::Overflow].set_handler(interrupt_overflow, type_attribute)?;
        IDT[InterruptVector::PageFault]
            .set_page_fault_handler(page_fault_handler, type_attribute)?;
        IDT[InterruptVector::Xhci].set_handler(interrupt_mouse_handler, type_attribute)?;
        IDT[InterruptVector::ApicTimer].set_handler(interrupt_timer_handler, type_attribute)?;
        IDT.load();
    }

    Ok(())
}


extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    serial_println!("{:?}", stack_frame);
    serial_println!("rip = {:?}", read_rip());
    common_lib::assembly::hlt_forever();
}
