use kernel_lib::println;
use mouse_driver::pci::configuration_space::device::find_usb_mouse;

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
fn it_not_over_flow_frame_buffer() {
    for i in 0..30 {
        println!("{}AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", i);
    }
}

#[test_case]
fn it_fetch_mouse_device() {
    assert!(find_usb_mouse().is_some())
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
