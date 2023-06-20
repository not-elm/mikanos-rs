use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use common_lib::queue::vector_queue::VectorQueue;
use kernel_lib::apic::LocalApicRegisters;

use crate::interrupt::InterruptMessage;

pub static mut INTERRUPT_QUEUE: VectorQueue<InterruptMessage> = VectorQueue::new();


pub extern "x86-interrupt" fn interrupt_xhci_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        INTERRUPT_QUEUE.enqueue(InterruptMessage::Xhci);
    }

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
}
