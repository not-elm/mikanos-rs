use pci::xhci::registers::capability_registers::CapabilityRegisters;

use crate::test_runner::xhci::mmio_base_addr;

#[test_case]
fn it_usb_command_register_default_value_zeros() {
    let value = unsafe {
        let ptr = base_usb_command_addr() as *const u32;
        *ptr
    };

    assert_eq!(value, 0);
}

fn base_usb_command_addr() -> usize {
    let mmio_base_addr = mmio_base_addr();
    let cp = CapabilityRegisters::new(mmio_base_addr).unwrap();
    mmio_base_addr + cp.cap_length.read_volatile() as usize
}
