use core::cell::RefCell;

use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use common_lib::queue::vector_queue::VectorQueue;
use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::serial_println;

use crate::interrupt::InterruptMessage;

pub static INTERRUPT_QUEUE: Mutex<RefCell<VectorQueue<InterruptMessage>>> =
    Mutex::new(RefCell::new(VectorQueue::new()));


pub extern "x86-interrupt" fn interrupt_mouse_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("Interrupt Mouse Handler");

    INTERRUPT_QUEUE
        .lock()
        .borrow_mut()
        .enqueue(InterruptMessage::Xhci);

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
}
