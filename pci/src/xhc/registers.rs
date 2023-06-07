use crate::xhc::registers::traits::capability_registers_accessible::CapabilityRegistersAccessible;
use crate::xhc::registers::traits::config_register_accessible::ConfigRegisterAccessible;
use crate::xhc::registers::traits::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::registers::traits::registers_operation::RegistersOperation;
use crate::xhc::registers::traits::usb_command_register_accessible::UsbCommandRegisterAccessible;

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
