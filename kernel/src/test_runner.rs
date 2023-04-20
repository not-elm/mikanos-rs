use common_lib::math::vector::Vector2D;
use kernel_lib::gop::console::fill_rect_using_global;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;

use crate::qemu::{exit_qemu, QemuExitCode};
use crate::serial_println;

mod macros_test;
mod msi;
mod xhci;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        serial_println!("test name={}", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
        serial_println!();
    }
}

#[cfg(test)]
pub fn my_runner(tests: &[&dyn Testable]) {
    fill_rect_using_global(
        Vector2D::new(0, 0),
        Vector2D::new(500, 500),
        PixelColor::new(0x00, 0x00, 0x00),
    )
        .unwrap();
    serial_println!("start test! num={}", tests.len());
    for t in tests {
        t.run();
    }
    serial_println!("============= end test! ================");
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn it_fetch_mouse_device() {
    assert!(PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .search()
        .is_some());
}
