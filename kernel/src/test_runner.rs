use common_lib::vector::Vector2D;
use kernel_lib::gop::console::fill_rect_using_global;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::println;

use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;

mod macros_test;
mod xhci;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        println!("test name={}", core::any::type_name::<T>());
        self();
        println!("[ok]");
        println!();
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
    println!("start test! num={}", tests.len());
    for t in tests {
        t.run();
    }
    println!("============= end test! ================");
    common_lib::assembly::hlt_forever();
}

#[test_case]
fn it_fetch_mouse_device() {
    assert!(PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .search()
        .is_some());
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }
//
// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;
//
//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }
