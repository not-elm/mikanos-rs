use crate::xhc::registers::external::External;
use crate::xhc::registers::traits::capability_registers_accessible::CapabilityRegistersAccessible;

impl<M> CapabilityRegistersAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn read_max_scratchpad_buffers_len(&self) -> usize {
        self.0
            .capability
            .hcsparams2
            .read_volatile()
            .max_scratchpad_buffers() as usize
    }
}
