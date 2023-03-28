use pci::xhc::registers::internal::capability_registers::structural_parameters1::StructuralParameters1;

use crate::serial_println;
use crate::test_runner::xhci::hcs_params1_offset;

#[test_case]
fn it_read_max_ports() {
    let ports = StructuralParameters1::new(hcs_params1_offset()).max_ports();
    serial_println!("max_ports={}", ports);
    assert!(0 < ports)
}