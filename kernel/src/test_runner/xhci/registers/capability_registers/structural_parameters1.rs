use pci::xhc::registers::capability_registers::structural_parameters1::StructuralParameters1;

use crate::{hcs1_offset, serial_println};

#[test_case]
fn it_read_max_ports() {
    let ports = StructuralParameters1::new(hcs1_offset()).max_ports();
    serial_println!("max_ports={}", ports);
    assert!(0 < ports)
}
