use core::num::NonZeroUsize;

use kernel_lib::interrupt::interrupt_queue_waiter::InterruptQueueWaiter;
use pci::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::error::PciResult;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::External;
use pci::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

use crate::interrupt::mouse::INTERRUPT_MOUSE_QUEUE;

pub fn start_xhci_host_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> PciResult {
    let external = External::new(mmio_base_addr, IdentityMapper());
    let mut xhc_controller = XhcController::new(
        external,
        MikanOSPciMemoryAllocator::new(),
        MouseDriverFactory::subscriber(mouse_subscriber),
    )?;

    xhc_controller.reset_port();

    let queue_waiter = unsafe { InterruptQueueWaiter::new(&mut INTERRUPT_MOUSE_QUEUE) };
    queue_waiter.for_each(|_| {
        xhc_controller.process_event();
    });

    Ok(())
}


#[derive(Clone, Debug)]
struct IdentityMapper();

impl xhci::accessor::Mapper for IdentityMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> NonZeroUsize {
        NonZeroUsize::new_unchecked(phys_start)
    }

    fn unmap(&mut self, _virtual_start: usize, _bytes: usize) {}
}
