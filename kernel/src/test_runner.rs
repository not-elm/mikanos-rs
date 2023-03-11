use kernel_lib::println;
use mouse_driver::assembly::io::{read_data, write_addr};

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
    common_lib::assembly::hlt_forever();
}

#[test_case]
fn it_should() {
    let data = mouse_driver::assembly::io::read_data();
    println!("data: {}", data);
    mouse_driver::assembly::io::write_addr(0);
    println!("data2 {}", mouse_driver::assembly::io::read_data());
    write_addr(36);

    println!("data3 {}", read_data());

    assert_ne!(data, 0)
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
