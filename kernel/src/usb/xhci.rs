use kernel_lib::interrupt::interrupt_message::TaskMessage;
use pci::class_driver::mouse::driver::MouseDriver;
use pci::class_driver::mouse::subscribable::MouseSubscribable;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::{External, IdentityMapper};
use pci::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

use crate::apic::TIMER_200_MILLI_INTERVAL;
use crate::interrupt::timer::TASK_MANAGER;
use crate::layers::LAYERS;
use crate::task::task_message_iter::TaskMessageIter;
use crate::usb::keyboard::build_keyboard_driver;

pub fn start_xhci_host_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<()> {
    unsafe {
        crate::task::init();
        TASK_MANAGER.set_interval(TIMER_200_MILLI_INTERVAL);
    }

    let mut xhc_controller = start_xhc_controller(mmio_base_addr, mouse_subscriber)?;

    let messages = TaskMessageIter::new(0);
    messages.for_each(|message| match message {
        TaskMessage::Xhci => {
            xhc_controller.process_all_events();
        }

        TaskMessage::Count { count, layer_key } => {
            update_count(count, &layer_key);
        }

        _ => {}
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


#[inline(always)]
fn update_count(count: usize, key: &str) {
    LAYERS
        .lock()
        .update_layer(key, |layer| {
            let window = layer.require_count().unwrap();
            window.write_count(count);
        })
        .unwrap();
}
