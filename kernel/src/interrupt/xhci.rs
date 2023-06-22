use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::interrupt::interrupt_message::TaskMessage;
use kernel_lib::serial_println;

use crate::task::TASK_MANAGER;

pub extern "x86-interrupt" fn interrupt_xhci_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        TASK_MANAGER
            .send_message_at(0, TaskMessage::Xhci)
            .unwrap();
    }

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
}
