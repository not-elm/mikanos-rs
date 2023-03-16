use kernel_lib::println;
use macros::Volatile;
use mouse_driver::pci::configuration_space::common_header::class_code::ClassCode;
use mouse_driver::pci::configuration_space::common_header::sub_class::Subclass;
use mouse_driver::pci::pci_device_searcher::PciDeviceSearcher;

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

#[derive(Volatile)]
#[volatile_type(u64)]
struct VolatileStruct(usize);

#[test_case]
fn it_impl_write_volatile() {
    let addr = [0x00u64; 3].as_ptr().addr();
    let v = VolatileStruct(addr);
    assert_eq!(v.read_volatile(), 0x00);
    v.write_volatile(0xFF);

    assert_eq!(v.read_volatile(), 0xFF);
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
