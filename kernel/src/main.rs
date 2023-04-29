#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(test_runner::my_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![feature(result_option_inspect)]
extern crate alloc;

use core::alloc::Layout;
use core::panic::PanicInfo;

use uefi::table::boot::MemoryMapIter;

use allocate::init_alloc;
use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::apic::device_config::LocalApicTimerDivide;
use kernel_lib::gop::console::init_console;
use kernel_lib::timer::apic::local_apic_timer::OneShotLocalApicTimer;
use kernel_lib::timer::apic::ApicTimer;
use kernel_lib::{println, serial_println};

use crate::gdt::init_gdt;
use crate::interrupt::init_idt;
use crate::layers::init_layers;
use crate::paging::init_paging_table;
use crate::usb::mouse::MouseSubscriber;
use crate::usb::xhci::start_xhci_host_controller;
use crate::usb::{enable_msi, first_general_header};

mod allocate;
mod entry_point;
mod gdt;
mod interrupt;
mod layers;
mod paging;
mod qemu;
#[cfg(test)]
mod test_runner;
mod usb;

#[cfg(test)]
macros::declaration_volatile_accessible!();

kernel_entry_point!();


#[no_mangle]
pub extern "sysv64" fn kernel_main(
    frame_buffer_config: &FrameBufferConfig,
    memory_map: &MemoryMapIter<'static>,
) {
    init_gdt();

    init_idt().unwrap();

    init_paging_table();

    init_alloc(memory_map.clone()).unwrap();

    init_console(*frame_buffer_config);

    init_layers(*frame_buffer_config).unwrap();


    #[cfg(test)]
    test_main();
    serial_println!("Hello Serial Port!");
    println!("Hello Kernel!");


    let mut timer = OneShotLocalApicTimer::new();

    timer.start(LocalApicTimerDivide::By1);

    let general_header = first_general_header();
    enable_msi(general_header.clone()).unwrap();

    start_xhci_host_controller(general_header.mmio_base_addr(), MouseSubscriber::new()).unwrap();

    common_lib::assembly::hlt_forever();
}


/// この関数はパニック時に呼ばれる
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("{:?}", info);
    common_lib::assembly::hlt_forever();
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[test failed!]");
    serial_println!("{}", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
}

#[alloc_error_handler]
fn on_oom(layout: Layout) -> ! {
    println!("Failed Heap Allocate! {:?}", layout);
    common_lib::assembly::hlt_forever();
}
