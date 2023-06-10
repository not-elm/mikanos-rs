use kernel_lib::interrupt::asm::sti;
use kernel_lib::timer::apic::timeout::Timeout;
use kernel_lib::timer::timer_manager::TimeOutManager;
use pci::class_driver::keyboard;
use pci::class_driver::keyboard::driver::KeyboardDriver;
use pci::class_driver::keyboard::subscribe::{KeyModifier, KeyboardSubscribable};
use pci::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::{External, IdentityMapper};
use pci::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

use crate::interrupt::interrupt_queue_waiter::InterruptQueueWaiter;
use crate::interrupt::InterruptMessage;
use crate::layers::{LAYERS, WINDOW_COUNT, WINDOW_LAYER_KEY};
use crate::println;


pub fn start_xhci_host_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<()> {
    sti();

    let mut xhc_controller = start_xhc_controller(mmio_base_addr, mouse_subscriber)?;
    let mut timer_manager = new_time_manager();
    let queue_waiter = InterruptQueueWaiter::new();

    queue_waiter.for_each(|message| match message {
        InterruptMessage::Xhci => {
            xhc_controller.process_event();
        }
        InterruptMessage::ApicTimer => {
            if let Some(timeouts) = timer_manager.tick() {
                timeouts
                    .iter()
                    .for_each(|timeout| {
                        update_count(*timeout as u32);

                        println!("Timeout = {}", timeout);
                    });
            }
        }
    });

    Ok(())
}


fn new_time_manager() -> TimeOutManager<usize> {
    let mut timer_manager = TimeOutManager::<usize>::default();
    timer_manager.push_timeout(Timeout::new(1, 1));
    timer_manager.push_timeout(Timeout::new(3, 3));
    timer_manager.push_timeout(Timeout::new(10, 10));
    timer_manager.push_timeout(Timeout::new(30, 30));
    timer_manager.push_timeout(Timeout::new(60, 60));

    timer_manager
}


fn update_count(count: u32) {
    LAYERS
        .layers_mut()
        .lock()
        .borrow_mut()
        .update_layer(WINDOW_COUNT, |layer| {
            let window = layer.require_count().unwrap();
            window.write_count(count as usize);
        })
        .unwrap();
}


fn start_xhc_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<XhcController<External<IdentityMapper>, MikanOSPciMemoryAllocator>> {
    let registers = External::new(mmio_base_addr, IdentityMapper);
    let allocator = MikanOSPciMemoryAllocator::new();
    let mouse_driver_factory = MouseDriverFactory::subscriber(mouse_subscriber);

    let mut xhc_controller = XhcController::new(
        registers,
        allocator,
        mouse_driver_factory,
        build_keyboard_driver(),
    )
    .map_err(|_| anyhow::anyhow!("Failed initialize xhc controller"))?;

    xhc_controller.reset_port();

    Ok(xhc_controller)
}


fn build_keyboard_driver() -> KeyboardDriver {
    keyboard::builder::Builder::new()
        .auto_upper_if_shift()
        .boxed_build(keyboard_subscribe)
}


fn keyboard_subscribe(
    _prev_key_modifiers: &[KeyModifier],
    _key_modifiers: &[KeyModifier],
    _prev_keycodes: &[char],
    _keycodes: &[char],
) {
    println!("{:?}", _keycodes);
}
