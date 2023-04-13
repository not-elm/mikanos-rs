use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use common_lib::queue::vector_queue::VectorQueue;
use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::serial_println;

pub static mut INTERRUPT_QUEUE: VectorQueue<u32> = VectorQueue::new();


pub extern "x86-interrupt" fn interrupt_mouse_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("+++++++++++++");

    unsafe {
        INTERRUPT_QUEUE.enqueue(32);
    }

    LocalApicRegisters::default().end_of_interrupt().eoi();
}


