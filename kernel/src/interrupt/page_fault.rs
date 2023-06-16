use crate::println;
use kernel_lib::serial_println;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    serial_println!("{:?}", stack_frame);

    common_lib::assembly::hlt_forever();
}
