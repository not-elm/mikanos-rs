use core::cell::RefCell;

use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use common_lib::queue::vector_queue::VectorQueue;
use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::serial_println;

pub static INTERRUPT_MOUSE_QUEUE: Mutex<RefCell<VectorQueue<()>>> =
    Mutex::new(RefCell::new(VectorQueue::new()));


pub extern "x86-interrupt" fn interrupt_mouse_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("Interrupt Mouse");

    INTERRUPT_MOUSE_QUEUE
        .lock()
        .borrow_mut()
        .enqueue(());

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
}
