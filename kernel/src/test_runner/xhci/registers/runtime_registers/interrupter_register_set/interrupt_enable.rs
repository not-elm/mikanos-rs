use pci::xhci::registers::runtime_registers::interrupter_register_set::interrupter_management_register::interrupt_enable::InterruptEnable;
use pci::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;

use crate::test_runner::xhci::registers::execute_reset_host_controller;
use crate::test_runner::xhci::registers::runtime_registers::interrupter_register_set_offset;

#[test_case]
fn it_access_correct_interrupt_enable() {
    execute_reset_host_controller();

    let offset = interrupter_register_set_offset();
    let ptr = offset.offset() as *const u8;
    assert_eq!(unsafe { *ptr } & 0b10, 0);

    assert!(InterruptEnable::new_check_flag_false(offset).is_ok())
}
