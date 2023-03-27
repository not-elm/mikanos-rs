use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::transfer::command_ring::CommandRing;
use crate::xhc::transfer::event_ring_table::EventRingTable;
use crate::xhc::transfer::ring::RingBase;
use crate::xhc::XhcRegistersHoldable;

pub struct XhciLibraryRegisters<M>(xhci::registers::Registers<M>)
where
    M: xhci::accessor::Mapper + Clone;

impl<M> XhciLibraryRegisters<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    pub fn new(mmio_addr: MemoryMappedAddr, mapper: M) -> Self {
        Self(unsafe { xhci::Registers::new(mmio_addr.addr(), mapper) })
    }
}

impl<M> XhciLibraryRegisters<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn registers_mut(&mut self) -> &mut xhci::registers::Registers<M> {
        &mut self.0
    }
}

impl<M> XhcRegistersHoldable for XhciLibraryRegisters<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn reset(&mut self) -> PciResult {
        let registers = self.registers_mut();
        registers.operational.usbcmd.update_volatile(|usb_cmd| {
            usb_cmd.clear_run_stop();
            usb_cmd.set_host_controller_reset();
        });

        while !registers.operational.usbsts.read_volatile().hc_halted() {}
        while registers
            .operational
            .usbsts
            .read_volatile()
            .controller_not_ready()
        {}

        Ok(())
    }

    fn run(&mut self) -> PciResult {
        self.0.operational.usbcmd.update_volatile(|u| {
            u.set_run_stop();
        });

        while self.0.operational.usbsts.read_volatile().hc_halted() {}

        let connect_index = self
            .0
            .port_register_set
            .into_iter()
            .position(|p| p.portsc.current_connect_status())
            .unwrap();

        self.0
            .port_register_set
            .update_volatile_at(connect_index, |p| {
                p.portsc.set_port_reset();
                p.portsc.set_wake_on_connect_enable();
            });

        while self
            .0
            .port_register_set
            .read_volatile_at(connect_index)
            .portsc
            .port_reset()
        {}

        Ok(())
    }

    fn setup_event_ring(&mut self, event_ring_table: &EventRingTable) -> PciResult {
        let registers = self.registers_mut();
        let mut primary_interrupter = registers.interrupter_register_set.interrupter_mut(0);

        primary_interrupter.erstba.update_volatile(|erstba| {
            erstba.set(event_ring_table.table_address());
        });

        primary_interrupter.erdp.update_volatile(|erdp| {
            let address = event_ring_table.event_ring_address();

            erdp.set_event_ring_dequeue_pointer(address)
        });

        Ok(())
    }

    fn setup_command_ring(&mut self, command_ring: &CommandRing) -> PciResult {
        let registers = self.registers_mut();
        registers.operational.crcr.update_volatile(|crcr| {
            crcr.set_ring_cycle_state();
            crcr.set_command_ring_pointer(command_ring.ring_base_addr());
        });
        Ok(())
    }

    fn setup_device_context_array(&mut self, a: &mut impl MemoryAllocatable) -> PciResult {
        todo!()
    }
}
