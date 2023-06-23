use kernel_lib::interrupt::asm::{cli, sti};
use kernel_lib::register::rflags::RFlags;

#[test_case]
fn it_read_interrupt_enable_flag() {
    cli();
    assert!(RFlags::read().are_enable_disabled());
    sti();
    assert!(RFlags::read().are_enable_interrupt());
}