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
use core::sync::atomic::{AtomicBool, Ordering};

use spin::RwLock;
use uefi::table::boot::MemoryMapIter;

use allocate::init_alloc;
use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::register::read::read_cr3;
use kernel_lib::serial_println;
use kernel_lib::task::AlignedTaskContext;

use crate::gdt::init_gdt;
use crate::interrupt::init_idt;
use crate::layers::init_layers;
use crate::paging::init_paging_table;
use crate::usb::{enable_msi, serial_bus_usb_devices};
use crate::usb::mouse::MouseSubscriber;
use crate::usb::xhci::start_xhci_host_controller;

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


static mut TASK_A_CTX: AlignedTaskContext = AlignedTaskContext::uninit();
static mut TASK_B_CTX: AlignedTaskContext = AlignedTaskContext::uninit();
static TASK_FLAG: AtomicBool = AtomicBool::new(true);

pub fn switch() {
    unsafe {
        let running_a = TASK_FLAG.load(Ordering::Relaxed);
        TASK_FLAG.store(!running_a, Ordering::Relaxed);
        serial_println!("running A = {}", running_a);
        if running_a {
            TASK_A_CTX.switch_to(&TASK_B_CTX);
        } else {
            TASK_B_CTX.switch_to(&TASK_A_CTX);
        }
    }
}

#[allow(clippy::fn_to_numeric_cast)]
fn init_task() {
    unsafe {
        let task_b_stack: [u64; 1024] = [0; 1024];
        let task_b_stack_end = task_b_stack
            .as_ptr_range()
            .end as u64;

        unsafe extern "sysv64" fn task(id: u64, data: u64) {
            let mut count = 0;

            loop {
                serial_println!("1. Start Task B id = {} data = {}", id, data);
                println!("Task B count = {}", count);
                count += 1;
            }
        }

        TASK_B_CTX.update(task as u64, (task_b_stack_end & !0xF) - 8);
    }
}


kernel_entry_point!();


#[no_mangle]
pub extern "sysv64" fn kernel_main(
    frame_buffer_config: &FrameBufferConfig,
    memory_map: &MemoryMapIter<'static>,
    rsdp: &Option<*const c_void>,
) {
    init_gdt();

    init_task();

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
