use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use kernel_lib::error::KernelResult;
use kernel_lib::interrupt::gate_type::GateType;
use kernel_lib::interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute;
use kernel_lib::interrupt::interrupt_vector::InterruptVector;
use kernel_lib::interrupt::IDT;

use crate::println;

use self::mouse::interrupt_mouse_handler;

pub mod mouse;

pub fn init_idt() -> KernelResult {
    unsafe {
        let type_attribute = InterruptDescriptorAttribute::new()
            .with_gate_type(GateType::InterruptGate)
            .with_present(true);

        IDT[0x08].set_page_fault_handler(page_fault_handler, type_attribute)?;
        IDT[InterruptVector::Xhci].set_handler(interrupt_mouse_handler, type_attribute)?;

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
    common_lib::assembly::hlt_forever();
}
