use crate::error::PciResult;
use crate::xhc::registers::external::External;
use crate::xhc::registers::traits::usb_command::UsbCommandRegisterAccessible;

impl<M> UsbCommandRegisterAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn write_command_ring_addr(&mut self, command_ring_addr: u64) -> PciResult {
        let registers = self.registers_mut();

        registers
            .operational
            .crcr
            .update_volatile(|crcr| {
                crcr.set_ring_cycle_state();

                crcr.set_command_ring_pointer(command_ring_addr);
            });

        Ok(())
    }
}
