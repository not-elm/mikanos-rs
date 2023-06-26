use core::fmt::{Debug, Formatter};



use crate::error::PciResult;
use crate::xhc::registers::external::External;
use crate::xhc::registers::traits::interrupter::InterrupterSetRegisterAccessible;

impl<M> InterrupterSetRegisterAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn write_event_ring_dequeue_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_addr: u64,
    ) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erdp
            .update_volatile(|erdp| erdp.set_event_ring_dequeue_pointer(event_ring_segment_addr));

        self.clear_event_handler_busy_at(index);

        Ok(())
    }


    fn clear_event_handler_busy_at(&mut self, index: usize) {
        self.0
            .interrupter_register_set
            .interrupter_mut(index)
            .erdp
            .update_volatile(|erdp| {
                erdp.clear_event_handler_busy();
            });
    }


    fn write_event_ring_segment_table_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_table_addr: u64,
    ) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erstba
            .update_volatile(|erstba| erstba.set(event_ring_segment_table_addr));

        Ok(())
    }


    fn write_interrupter_enable_at(&mut self, index: usize, is_enable: bool) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                if is_enable {
                    iman.set_interrupt_enable();
                } else {
                    iman.clear_interrupt_enable();
                }
            });

        Ok(())
    }


    fn write_interrupter_pending_at(&mut self, index: usize, is_pending: bool) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                if is_pending {
                    iman.set_0_interrupt_pending();
                } else {
                    iman.clear_interrupt_pending();
                }
            });

        Ok(())
    }


    fn read_dequeue_pointer_addr_at(&mut self, index: usize) -> u64 {
        self.0
            .interrupter_register_set
            .interrupter(index)
            .erdp
            .read_volatile()
            .event_ring_dequeue_pointer()
    }


    fn write_event_ring_segment_table_size(&mut self, index: usize, size: u16) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erstsz
            .update_volatile(|e| e.set(size));

        Ok(())
    }


    fn set_counter_at(&mut self, index: usize, count: u16) {
        self.0
            .interrupter_register_set
            .interrupter_mut(index)
            .imod
            .update_volatile(|imod| {
                imod.set_interrupt_moderation_counter(count);
            });
    }


    fn clear_interrupt_pending_at(&mut self, index: usize) {
        self.0
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                iman.clear_interrupt_pending();
            });
    }
}


impl<M> Debug for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let busy = self
            .0
            .interrupter_register_set
            .interrupter(0)
            .erdp
            .read_volatile()
            .event_handler_busy();

        let enable = self
            .0
            .interrupter_register_set
            .interrupter(0)
            .iman
            .read_volatile()
            .interrupt_enable();

        let pending = self
            .0
            .interrupter_register_set
            .interrupter(0)
            .iman
            .read_volatile()
            .interrupt_pending();

        let counter = self
            .0
            .interrupter_register_set
            .interrupter(0)
            .imod
            .read_volatile()
            .interrupt_moderation_counter();
        let event_interrupt = self
            .0
            .operational
            .usbsts
            .read_volatile()
            .event_interrupt();

        f.debug_struct("External")
            .field("event_handler_busy", &busy)
            .field("interrupt_enable", &enable)
            .field("interrupt_pending", &pending)
            .field("interrupt_counter", &counter)
            .field("event_interrupt", &event_interrupt)
            .finish()
    }
}
