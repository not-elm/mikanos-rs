use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::serial_println;

use crate::println;

pub extern "x86-interrupt" fn interrupt_overflow(stack_frame: InterruptStackFrame) {
    println!("***** Interrupt Exception OverFlow!!! *****");

    serial_println!("{:?}", stack_frame);
    println!("{:?}", stack_frame);

    common_lib::assembly::hlt_forever();
}