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
use kernel_lib::gop::console::{fill_rect_using_global, init_console};
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::println;
use macros::declaration_volatile_accessible;
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;
use pci::xhci::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhci::registers::capability_registers::capability_length::CapabilityLength;
use pci::xhci::registers::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use pci::xhci::registers::capability_registers::structural_parameters1::StructuralParameters1Offset;
use pci::xhci::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use pci::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;
use pci::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;
use pci::xhci::registers::runtime_registers::RuntimeRegistersOffset;
use pci::xhci::registers::Registers;

mod qemu;
mod serial;
#[cfg(test)]
mod test_runner;
declaration_volatile_accessible!();

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) {
    init_console(frame_buffer_config);
    println!("hello!");
    use pci::xhci::allocator::memory_allocatable::MemoryAllocatable;

    #[cfg(test)]
    test_main();
    serial_println!("hello Serial!");

    let registers = Registers::new(mmio_base_addr()).unwrap();

    let ps = registers.port_registers();

    let mut event_ring = registers
        .init(&mut MikanOSPciMemoryAllocator::new())
        .unwrap();
    registers.run();

    for p in ps {
        p.reset();
        println!("port = {:?}", p);
    }
    loop {
        registers.a();
    }
    // reset_controller(
    //     &HostControllerHalted::new(usb_status_register_offset()).unwrap(),
    //     &HostControllerReset::new(operation_registers_offset()),
    //     &ControllerNotReady::new(usb_status_register_offset()).unwrap(),
    // )
    // .unwrap();

    // fill_background(PixelColor::new(0x3E, 0x3E, 0x3E), &frame_buffer_config).unwrap();
    // fill_bottom_bar(PixelColor::new(0x00, 0x00, 0xFF), &frame_buffer_config).unwrap();
    //
    // draw_cursor().unwrap();

    // let mmio_base_addr = PciDeviceSearcher::new()
    //     .class_code(ClassCode::SerialBus)
    //     .sub_class(Subclass::Usb)
    //     .search()
    //     .unwrap()
    //     .cast_device()
    //     .expect_single()
    //     .unwrap()
    //     .expect_general()
    //     .unwrap()
    //     .mmio_base_addr();
    //
    // let cap_length = CapabilityLength::new(mmio_base_addr).unwrap();
    //
    // let addr =
    //     UsbStatusRegisterOffset::new(OperationRegistersOffset::new(mmio_base_addr, cap_length))
    //         .offset();
    // println!("operation_registers_addr = {:x}", addr);
    // let data = unsafe { core::ptr::read_volatile((addr as *const u32)) };
    // println!("mmio_base_addr = {:b}", data);
    // let data = unsafe { core::ptr::read_volatile((addr as *const u8)) };
    // println!("mmio_base_addr = {:b}", data);

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
