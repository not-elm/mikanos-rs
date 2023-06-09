use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use kernel_lib::apic::LocalApicRegisters;

use crate::interrupt::InterruptMessage;
use crate::interrupt::mouse::INTERRUPT_QUEUE;

pub extern "x86-interrupt" fn interrupt_timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        INTERRUPT_QUEUE
            .enqueue(InterruptMessage::ApicTimer);
    }
    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
}
