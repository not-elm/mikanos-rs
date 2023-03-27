#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(test_runner::my_runner)]
#![reexport_test_harness_main = "test_main"]

use core::num::NonZeroUsize;
use core::ops::{Add, DerefMut};
use core::panic::PanicInfo;

use uefi::table::boot::{MemoryMapIter, MemoryType};
use x86_64::structures::paging::mapper::CleanUp;
use x86_64::structures::paging::{FrameAllocator, Mapper, PageSize, Translate};

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop::console::{fill_rect_using_global, init_console};
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::segment::setup_segments;
use kernel_lib::{println, serial_println};
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::capability_registers::capability_length::CapabilityLength;
use pci::xhc::registers::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use pci::xhc::registers::capability_registers::structural_parameters1::StructuralParameters1Offset;
use pci::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use pci::xhc::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;
use pci::xhc::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;
use pci::xhc::registers::runtime_registers::RuntimeRegistersOffset;
use pci::xhc::xhci_library_registers::XhciLibraryRegisters;
use pci::xhc::{XhcController, XhcRegistersHoldable};

mod qemu;
#[cfg(test)]
mod test_runner;

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
    unsafe { setup_segments() };

    #[cfg(test)]
    test_main();
    serial_println!("hello Serial!");

    fill_background(PixelColor::new(0, 0, 0x22), frame_buffer_config).unwrap();
    fill_bottom_bar(PixelColor::new(0, 0, 0xFF), frame_buffer_config).unwrap();
    serial_println!("MMIO ADDRESS = {:x}", mmio_base_addr().addr());
    // let mut registers = pci::xhc::registers::Registers::new(mmio_base_addr()).unwrap();
    // registers
    //     .init(&mut MikanOSPciMemoryAllocator::new())
    //     .unwrap();
    //

    // let rs = unsafe{xhci::registers::Registers::new(mmio_base_addr().addr(), IdentityMapper())};
    let mut rs = XhciLibraryRegisters::new(mmio_base_addr(), IdentityMapper());
    let mut xhc_controller =
        XhcController::new(&mut rs, &mut MikanOSPciMemoryAllocator::new()).unwrap();

    xhc_controller.start_event_pooling();
    // xhc_controller.start_event_pooling();

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

#[allow(dead_code)]
fn runtime_registers_offset() -> RuntimeRegistersOffset {
    let rts_off = RuntimeRegisterSpaceOffset::new_with_check_size(
        mmio_base_addr(),
        &CapabilityLength::new_check_length(mmio_base_addr()).unwrap(),
    )
    .unwrap();

    RuntimeRegistersOffset::new(mmio_base_addr(), &rts_off)
}

#[allow(dead_code)]
fn interrupter_register_set_offset(index: usize) -> InterrupterRegisterSetOffset {
    InterrupterRegisterSetOffset::new(runtime_registers_offset(), index)
}

#[allow(dead_code)]
fn hcs1_offset() -> StructuralParameters1Offset {
    StructuralParameters1Offset::new(mmio_base_addr())
}

#[allow(dead_code)]
fn operation_registers_offset() -> OperationalRegistersOffset {
    let mmio_base_addr = mmio_base_addr();
    let cap_length = CapabilityLength::new_check_length(mmio_base_addr).unwrap();
    OperationalRegistersOffset::new(mmio_base_addr, &cap_length)
}

#[allow(dead_code)]
fn usb_status_register_offset() -> UsbStatusRegisterOffset {
    let mmio_addr = mmio_base_addr();
    let cap_length = CapabilityLength::new_check_length(mmio_addr).unwrap();
    UsbStatusRegisterOffset::new(OperationalRegistersOffset::new(mmio_addr, &cap_length))
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
