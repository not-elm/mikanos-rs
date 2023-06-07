use crate::xhc::registers::traits::capability::CapabilityRegistersAccessible;
use crate::xhc::registers::traits::config::ConfigRegisterAccessible;
use crate::xhc::registers::traits::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::interrupter::InterrupterSetRegisterAccessible;
use crate::xhc::registers::traits::port::PortRegistersAccessible;
use crate::xhc::registers::traits::registers_operation::RegistersOperation;
use crate::xhc::registers::traits::usb_command::UsbCommandRegisterAccessible;

pub mod external;
// pub mod internal;
pub mod memory_mapped_addr;
pub mod traits;


pub trait XhcRegisters:
    RegistersOperation
    + CapabilityRegistersAccessible
    + InterrupterSetRegisterAccessible
    + UsbCommandRegisterAccessible
    + DoorbellRegistersAccessible
    + PortRegistersAccessible
    + ConfigRegisterAccessible
    + DeviceContextBaseAddressArrayPointerAccessible
{
}
