use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::timer::TIME_HANDLE_MANAGER;

pub extern "x86-interrupt" fn interrupt_timer_handler(_stack_frame: InterruptStackFrame) {
    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();

    TIME_HANDLE_MANAGER.tick();
}
