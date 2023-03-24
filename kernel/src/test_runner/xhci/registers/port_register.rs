use crate::{operation_registers_offset, serial_println};
use pci::xhci::registers::port_registers::port::{Port, PortRegisterAddr};

#[test_case]
fn it_current_connect() {
    let is_connect = Port::new(PortRegisterAddr::new(operation_registers_offset(), 0)).is_connect();

    serial_println!("is_connect_port0={}", is_connect);
}
