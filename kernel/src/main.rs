#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(test_runner::my_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop::console::{draw_cursor, fill_rect_using_global, init_console};
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::println;
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;

#[cfg(test)]
mod test_runner;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    init_console(frame_buffer_config);
    println!("hello!");

    #[cfg(test)]
    test_main();

    fill_background(PixelColor::new(0x3E, 0x3E, 0x3E), &frame_buffer_config).unwrap();
    fill_bottom_bar(PixelColor::new(0x00, 0x00, 0xFF), &frame_buffer_config).unwrap();

    draw_cursor().unwrap();

    let mmio_base_addr = PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .search()
        .unwrap()
        .cast_device()
        .expect_single()
        .unwrap()
        .expect_general()
        .unwrap()
        .mmio_base_addr();

    println!("mmio_base_addr = {:x}", mmio_base_addr);

    common_lib::assembly::hlt_forever();
}

fn fill_background(color: PixelColor, config: &FrameBufferConfig) -> KernelResult {
    fill_rect_using_global(
        Vector2D::new(0, 0),
        Vector2D::new(config.horizontal_resolution, config.vertical_resolution),
        color,
    )
}

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
    println!("{:?}", info);
    common_lib::assembly::hlt_forever();
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    println!("[test failed!]");
    println!("{:?}", info);
    common_lib::assembly::hlt_forever();
}
