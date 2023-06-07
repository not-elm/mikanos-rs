use crate::error::PciResult;
use crate::xhc::registers::external::External;
use crate::xhc::registers::traits::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;

impl<M> DeviceContextBaseAddressArrayPointerAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) -> PciResult {
        self.registers_mut()
            .operational
            .dcbaap
            .update_volatile(|device_context| device_context.set(device_context_addr));

        Ok(())
    }
}
