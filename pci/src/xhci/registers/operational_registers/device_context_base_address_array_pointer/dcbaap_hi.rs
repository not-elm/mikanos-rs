use macros::VolatileBits;

use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerOffset;

#[derive(VolatileBits)]
#[volatile_type(u32)]
pub struct DcbaapHi(usize);

impl DcbaapHi {
    pub fn new(offset: DeviceContextBaseAddressArrayPointerOffset) -> Self {
        Self::new_uncheck(offset.offset() + 32)
    }
}
