use alloc::string::ToString;
use core::fmt::Write;

use kernel_lib::serial_println;
use kernel_lib::task::TaskManager;
use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;
use pci::class_driver::mouse::driver::MouseDriver;
use pci::class_driver::mouse::subscribable::MouseSubscribable;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::{External, IdentityMapper};
use pci::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

use crate::apic::TIMER_500_MILLI_INTERVAL;
use crate::interrupt::interrupt_queue_waiter::InterruptQueueWaiter;
use crate::interrupt::timer::TIMER;
use crate::layers::{KEYBOARD_TEXT, LAYERS};
use crate::task::TASK_MANAGER;

pub fn start_xhci_host_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<()> {
    let mut xhc_controller = start_xhc_controller(mmio_base_addr, mouse_subscriber)?;

    let queue_waiter = InterruptQueueWaiter::new();

    unsafe {
        crate::task::init();
        TIMER.set(TIMER_500_MILLI_INTERVAL);
    }

    queue_waiter.for_each(|_| {
        serial_println!("process_event ");
        xhc_controller.process_event();
    });

    Ok(())
}


fn start_xhc_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<XhcController<External<IdentityMapper>, MikanOSPciMemoryAllocator>> {
    let registers = External::new(mmio_base_addr, IdentityMapper);
    let allocator = MikanOSPciMemoryAllocator::new();

    let mut xhc_controller = XhcController::new(
        registers,
        allocator,
        MouseDriver::new(mouse_subscriber),
        build_keyboard_driver(),
    )
        .map_err(|_| anyhow::anyhow!("Failed initialize xhc controller"))?;

    xhc_controller
        .reset_port()
        .map_err(|e| e.inner())?;

    Ok(xhc_controller)
}


fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(_modifier_bits: u8, keycode: char) {
    LAYERS
        .layers_mut()
        .lock()
        .borrow_mut()
        .update_layer(KEYBOARD_TEXT, |layer| {
            layer
                .require_text()
                .unwrap()
                .write_str(keycode.to_string().as_str())
                .unwrap();
        })
        .unwrap();

    unsafe {
        if keycode == 's' {
            TASK_MANAGER.sleep_at(1).unwrap();
        } else if keycode == 'w' {
            TASK_MANAGER.wakeup_at(1).unwrap();
        }
    }
}
