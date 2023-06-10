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


use core::ffi::c_void;
use core::panic::PanicInfo;

use uefi::table::boot::MemoryMapIter;

use allocate::init_alloc;
use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::serial_println;

use crate::gdt::init_gdt;
use crate::interrupt::init_idt;
use crate::layers::init_layers;
use crate::paging::init_paging_table;
use crate::usb::mouse::MouseSubscriber;
use crate::usb::xhci::start_xhci_host_controller;
use crate::usb::{enable_msi, serial_bus_usb_devices};

mod allocate;
mod apic;
mod entry_point;
mod gdt;
mod interrupt;
mod layers;
mod paging;
mod qemu;
#[cfg(test)]
mod test_runner;
mod usb;


kernel_entry_point!();


#[no_mangle]
pub extern "sysv64" fn kernel_main(
    frame_buffer_config: &FrameBufferConfig,
    memory_map: &MemoryMapIter<'static>,
    rsdp: &Option<*const c_void>,
) {
    init_gdt();

    init_idt().unwrap();

    init_paging_table();

    init_alloc(memory_map.clone()).unwrap();

    init_layers(*frame_buffer_config).unwrap();

    apic::start_timer(*rsdp).unwrap();

    #[cfg(test)]
    test_main();
    serial_println!("Hello Serial Port!");
    println!("Hello Mikan OS RS!");

    let devices = serial_bus_usb_devices();
    let xhc_general_header = devices.first().unwrap();

    enable_msi(xhc_general_header.clone()).unwrap();

    start_xhci_host_controller(xhc_general_header.mmio_base_addr(), MouseSubscriber::new())
        .unwrap();

    common_lib::assembly::hlt_forever();
}


/// この関数はパニック時に呼ばれる
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);

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
fn on_oom(layout: core::alloc::Layout) -> ! {
    println!("Failed Heap Allocate! {:?}", layout);
    serial_println!("Failed Heap Allocate! {:?}", layout);
    common_lib::assembly::hlt_forever();
}
