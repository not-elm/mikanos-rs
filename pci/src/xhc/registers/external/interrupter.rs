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

        Ok(())
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
}
