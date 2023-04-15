#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(test_runner::my_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
extern crate alloc;

use core::alloc::Layout;
use core::panic::PanicInfo;

use uefi::table::boot::MemoryMapIter;

use allocate::init_alloc;
use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop::console::{fill_rect_using_global, init_console, CONSOLE_BACKGROUND_COLOR};
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::{println, serial_println};

use crate::interrupt::init_idt;
use crate::usb::mouse::MouseSubscriber;
use crate::usb::xhci::start_xhci_host_controller;
use crate::usb::{enable_msi, first_general_header};

pub mod allocate;
mod entry_point;
mod interrupt;
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
    _memory_map: &MemoryMapIter,
) {
    // unsafe { setup_segments() };

    init_alloc();

    init_console(*frame_buffer_config);

    init_idt().unwrap();


    #[cfg(test)]
    test_main();
    serial_println!("Hello Serial Port!");
    println!("Hello Kernel!");

    fill_background(CONSOLE_BACKGROUND_COLOR, frame_buffer_config).unwrap();
    fill_bottom_bar(PixelColor::new(0, 0, 0xFF), frame_buffer_config).unwrap();

    let general_header = first_general_header();
    enable_msi(general_header.clone()).unwrap();

    start_xhci_host_controller(
        general_header.mmio_base_addr(),
        MouseSubscriber::new(
            frame_buffer_config.horizontal_resolution,
            frame_buffer_config.vertical_resolution,
        ),
    )
    .unwrap();


    common_lib::assembly::hlt_forever();
}


#[allow(dead_code)]
fn fill_background(color: PixelColor, config: &FrameBufferConfig) -> KernelResult {
    fill_rect_using_global(
        Vector2D::new(0, 0),
        Vector2D::new(config.horizontal_resolution, config.vertical_resolution),
        color,
    )
}

#[allow(dead_code)]
fn fill_bottom_bar(color: PixelColor, config: &FrameBufferConfig) -> KernelResult {
    let v = config.vertical_resolution;
    let h = config.horizontal_resolution;
    fill_rect_using_global(Vector2D::new(0, v - 50), Vector2D::new(h, v), color)?;
    fill_rect_using_global(
        Vector2D::new(0, v - 50),
        Vector2D::new(50, v),
        PixelColor::new(0x33, 0x33, 0xAA),
    )
}


/// この関数はパニック時に呼ばれる
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
