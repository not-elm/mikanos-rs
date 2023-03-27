use kernel_lib::{println, serial_println};

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::transfer::event_ring_table::EventRingTable;
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

        serial_println!(
            "{:x}",
            registers
                .interrupter_register_set
                .interrupter_mut(0)
                .erdp
                .read_volatile()
                .event_ring_dequeue_pointer()
        );

        // registers.operational.usbcmd.update_volatile(|usb_cmd| {
        //     usb_cmd.clear_run_stop();
        // });

        while !registers.operational.usbsts.read_volatile().hc_halted() {}
        registers.operational.usbcmd.update_volatile(|usb_cmd| {
            usb_cmd.set_host_controller_reset();
            usb_cmd.set_light_host_controller_reset();
        });
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
        println!(
            "USB STATUS = {:?}",
            self.registers_mut()
                .interrupter_register_set
                .interrupter(0)
                .iman
                .read_volatile()
        );
        Ok(())
    }

    fn setup_event_ring(
        &mut self,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<(u64, u64)> {
        let registers = self.registers_mut();
        let mut primary_interrupter = registers.interrupter_register_set.interrupter_mut(0);
        let event_ring_table_addr = allocator.try_allocate_trb_ring(1)?;
        let event_ring_addr = allocator.try_allocate_trb_ring(32)?;

        primary_interrupter.iman.update_volatile(|iman| {
            iman.clear_interrupt_enable();
            iman.clear_interrupt_pending();
        });

        let _event_ring_table = EventRingTable::new(event_ring_table_addr, event_ring_addr)?;

        primary_interrupter.erstsz.update_volatile(|e| e.set(1));

        primary_interrupter
            .erdp
            .update_volatile(|erdp| erdp.set_event_ring_dequeue_pointer(event_ring_addr));

        primary_interrupter.erstba.update_volatile(|erstba| {
            erstba.set(event_ring_table_addr);
        });

        primary_interrupter.iman.update_volatile(|iman| {
            iman.set_0_interrupt_pending();
            iman.set_interrupt_enable();
        });
        registers.operational.usbcmd.update_volatile(|sts| {
            sts.set_interrupter_enable();
        });

        Ok((
            primary_interrupter.erstba.read_volatile().get(),
            primary_interrupter
                .erdp
                .read_volatile()
                .event_ring_dequeue_pointer(),
        ))
    }

    fn setup_command_ring(
        &mut self,
        ring_size: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<u64> {
        let registers = self.registers_mut();
        let command_ring_addr = allocator.try_allocate_trb_ring(ring_size)?;
        registers.operational.crcr.update_volatile(|crcr| {
            crcr.set_ring_cycle_state();
            crcr.set_command_ring_pointer(command_ring_addr);
        });
        Ok(command_ring_addr & !0b111111)
    }

    fn setup_device_context_array(&mut self, a: &mut impl MemoryAllocatable) -> PciResult {
        let registers = self.registers_mut();
        let max_slots = registers
            .capability
            .hcsparams1
            .read_volatile()
            .number_of_device_slots();
        println!("Event Ring Max Segment Size = {}", max_slots);

        let d = a.try_allocate_trb_ring(1024)?;

        registers
            .operational
            .dcbaap
            .update_volatile(|device_context| device_context.set(d));
        Ok(())
    }
}
