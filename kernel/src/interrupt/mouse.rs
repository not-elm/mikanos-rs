use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use common_lib::queue::vector_queue::VectorQueue;
use kernel_lib::apic::LocalApicRegisters;

pub static mut INTERRUPT_MOUSE_QUEUE: VectorQueue<()> = VectorQueue::new();


pub extern "x86-interrupt" fn interrupt_mouse_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        INTERRUPT_MOUSE_QUEUE.enqueue(());
    }

    LocalApicRegisters::default()
        .end_of_interrupt()
        .eoi();
}
