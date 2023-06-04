use core::num::NonZeroUsize;

use kernel_lib::apic::device_config::LocalApicTimerDivide;
use kernel_lib::interrupt::asm::sti;
use kernel_lib::timer::apic::local_apic_timer::LocalApicTimer;
use kernel_lib::timer::apic::timeout::Timeout;
use kernel_lib::timer::apic::ApicTimer;
use kernel_lib::timer::timer_manager::TimeOutManager;
use pci::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::External;
use pci::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

use crate::interrupt::interrupt_queue_waiter::InterruptQueueWaiter;
use crate::interrupt::InterruptMessage;
use crate::layers::{LAYERS, WINDOW_LAYER_KEY};
use crate::println;

pub fn start_xhci_host_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<()> {
    let external = External::new(mmio_base_addr, IdentityMapper);
    sti();

    let mut xhc_controller = XhcController::new(
        external,
        MikanOSPciMemoryAllocator::new(),
        MouseDriverFactory::subscriber(mouse_subscriber),
    )
    .map_err(|_| anyhow::anyhow!("Failed initialize xhc controller"))?;

    xhc_controller.reset_port();

    let mut timer_manager = new_time_manager();

    let mut timer = LocalApicTimer::new();
    timer.start(LocalApicTimerDivide::By1);

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
                        println!("Timeout = {}", timeout);
                    });
            }

            update_count(timer.elapsed());
        }
    });

    Ok(())
}


fn new_time_manager() -> TimeOutManager<usize> {
    let mut timer_manager = TimeOutManager::<usize>::default();
    timer_manager.push_timeout(Timeout::new(1, 1));
    timer_manager.push_timeout(Timeout::new(3, 3));

    timer_manager
}


fn update_count(count: u32) {
    LAYERS
        .layers_mut()
        .lock()
        .borrow_mut()
        .update_layer(WINDOW_LAYER_KEY, |layer| {
            let window = layer
                .require_window()
                .unwrap();
            window.write_count(count as usize);
        })
        .unwrap();
}


#[derive(Clone, Debug)]
struct IdentityMapper;

impl xhci::accessor::Mapper for IdentityMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> NonZeroUsize {
        NonZeroUsize::new_unchecked(phys_start)
    }

    fn unmap(&mut self, _virtual_start: usize, _bytes: usize) {}
}
