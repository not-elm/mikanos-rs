use kernel_lib::println;
use pci::xhci::registers::capability_registers::capability_length::CapabilityLength;
use pci::xhci::registers::capability_registers::CapabilityRegisters;
use pci::xhci::registers::operation_registers::operation_registers_offset::OperationRegistersOffset;
use pci::xhci::registers::operation_registers::usb_command_register::run_stop::RunStop;
use pci::xhci::registers::operation_registers::usb_status_register::hc_halted::HCHalted;
use pci::xhci::registers::operation_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

use crate::test_runner::xhci::mmio_base_addr;

// #[test_case]
// fn it_operation_registers_default_value_is_one() {
//     let value = unsafe { *(usb_status_register_offset().offset() as *const u32) };
//     println!("V{:b}", value);
//     assert_eq!(value, 1);
// }

#[test_case]
fn it_hc_halted() {
    println!("{:?}",);
    let hc = HCHalted::new(usb_status_register_offset().offset());
    hc.write_volatile(true);
    if !CapabilityLength::new(mmio_base_addr()).read_volatile() {
        RunStop
    }

    assert!(HCHalted::new_expect_halted(usb_status_register_offset()).is_ok());
}

fn usb_status_register_offset() -> UsbStatusRegisterOffset {
    let mmio_base_addr = mmio_base_addr();
    let cp = CapabilityLength::new_with_check(mmio_base_addr).unwrap();
    UsbStatusRegisterOffset::new(OperationRegistersOffset::new(mmio_base_addr, cp.cap_length))
}
