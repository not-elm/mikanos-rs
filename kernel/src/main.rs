#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(test_runner::my_runner)]
#![reexport_test_harness_main = "test_main"]

use core::num::NonZeroUsize;
use core::panic::PanicInfo;

use uefi::table::boot::{MemoryMapIter, MemoryType};

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop::console::{fill_rect_using_global, init_console};
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::{println, serial_println};
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::External;
use pci::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

mod qemu;
#[cfg(test)]
mod test_runner;
#[cfg(test)]
macros::declaration_volatile_accessible!();
// #[no_mangle]
// pub extern "sysv64" fn kernel_entry_point(
//     frame_buffer_config: &FrameBufferConfig,
//     memory_map: &MemoryMapIter,
// ) {
//     let address = KERNEL_STACK.end_addr();
//     serial_println!("address={:x}", address);
//     unsafe {
//         asm!(
//             "mov rsp, {0}",
//             "call kernel_main",
//
//             in(reg) address,
//             in("rdi") frame_buffer_config,
//             in("esi") memory_map,
//             clobber_abi("sysv64")
//         )
//     }
// }

#[no_mangle]
pub extern "sysv64" fn kernel_main(
    frame_buffer_config: &FrameBufferConfig,
    _memory_map: &MemoryMapIter,
) {
    init_console(*frame_buffer_config);
    // unsafe { setup_segments() };

    #[cfg(test)]
    test_main();
    serial_println!("Hello Serial Port!");
    println!("Hello Kernel!");

    fill_background(PixelColor::new(0, 0, 0x22), frame_buffer_config).unwrap();
    fill_bottom_bar(PixelColor::new(0, 0, 0xFF), frame_buffer_config).unwrap();
    serial_println!("MMIO ADDRESS = {:x}", mmio_base_addr().addr());

    let external = External::new(mmio_base_addr(), IdentityMapper());
    let mut xhc_controller =
        XhcController::new(external, MikanOSPciMemoryAllocator::new()).unwrap();

    xhc_controller.start_event_pooling().unwrap();

    common_lib::assembly::hlt_forever();
}

#[derive(Clone)]
struct IdentityMapper();

impl xhci::accessor::Mapper for IdentityMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> NonZeroUsize {
        return NonZeroUsize::new_unchecked(phys_start);
    }

    fn unmap(&mut self, _virt_start: usize, _bytes: usize) {}
}

#[allow(dead_code)]
fn is_available(memory_type: MemoryType) -> bool {
    match memory_type {
        MemoryType::BOOT_SERVICES_CODE
        | MemoryType::BOOT_SERVICES_DATA
        | MemoryType::MMIO
        | MemoryType::MMIO_PORT_SPACE
        | MemoryType::CONVENTIONAL => true,
        _ => false,
    }
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

#[allow(dead_code)]
fn mmio_base_addr() -> MemoryMappedAddr {
    let mouse = PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .search()
        .unwrap()
        .cast_device()
        .expect_single()
        .unwrap()
        .expect_general()
        .unwrap();

    mouse.mmio_base_addr()
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
