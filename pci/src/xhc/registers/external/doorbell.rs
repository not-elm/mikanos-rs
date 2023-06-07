use crate::error::PciResult;
use crate::xhc::registers::external::External;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

impl<M> DoorbellRegistersAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn notify_at(&mut self, index: usize, target: u8, stream_id: u16) -> PciResult {
        self.registers_mut()
            .doorbell
            .update_volatile_at(index, |doorbell| {
                doorbell.set_doorbell_target(target);
                doorbell.set_doorbell_stream_id(stream_id);
            });

        Ok(())
    }
}
