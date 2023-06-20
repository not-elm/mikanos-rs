use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use kernel_lib::interrupt::asm::cli;
use kernel_lib::serial_println;

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    cli();
    serial_println!("EXCEPTION: PAGE FAULT");
    serial_println!("Accessed Address: {:?}", Cr2::read());
    serial_println!("Error Code: {:?}", error_code);

    serial_println!("{:?}", stack_frame);

    common_lib::assembly::hlt_forever();
}
