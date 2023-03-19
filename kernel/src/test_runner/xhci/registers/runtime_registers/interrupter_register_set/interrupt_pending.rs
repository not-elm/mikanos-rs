use pci::xhci::registers::runtime_registers::interrupter_register_set::interrupter_management_register::interrupt_pending::InterruptPending;
use pci::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;

use crate::serial_println;
use crate::test_runner::xhci::registers::execute_reset_host_controller;
use crate::test_runner::xhci::registers::runtime_registers::interrupter_register_set_offset;

#[test_case]
fn it_access_correct_interrupt_pending() {
    execute_reset_host_controller();
    let o = interrupter_register_set_offset();
    serial_println!("offset={:b}", o.offset());
    assert!(InterruptPending::new_check_flag_false(o).is_ok())
}
