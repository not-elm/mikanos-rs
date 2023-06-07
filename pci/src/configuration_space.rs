use kernel_lib::io::asm::{fetch_config_data, write_config_addr};
use kernel_lib::io::config_address_register::ConfigAddrRegister;

use crate::configuration_space::common_header::class_code::ClassCode;
use crate::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::configuration_space::common_header::sub_class::Subclass;
use crate::configuration_space::device::function::Function;
use crate::configuration_space::device::function::multiple_function_device::MultipleFunctionDevice;
use crate::configuration_space::device::function::single_function_device::SingleFunctionDevice;
use crate::configuration_space::device::header_type::general_header::GeneralHeader;
use crate::configuration_space::device::header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;

pub mod common_header;
pub mod device;
pub mod msi;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ConfigurationSpace {
    bus: u8,
    device_slot: u8,
    function: u8,
}


impl ConfigurationSpace {
    pub fn try_new(bus: u8, device_slot: u8, function: u8) -> Option<Self> {
        let me = ConfigurationSpace::new(bus, device_slot, function);
        if me.vendor_id().valid_device() {
            Some(me)
        } else {
            None
        }
    }

    pub fn cast_device(self) -> Function {
        if self
            .header_type()
            .is_multiple_function()
        {
            Function::Multiple(MultipleFunctionDevice::new(self))
        } else {
            select_single_function_device(self)
        }
    }


    pub fn bus(&self) -> u8 {
        self.bus
    }


    pub fn device_slot(&self) -> u8 {
        self.device_slot
    }


    pub fn function(&self) -> u8 {
        self.function
    }


    pub(crate) fn fetch_data_offset_at(&self, offset: u8) -> u32 {
        write_config_addr(self.config_addr_at(offset));
        fetch_config_data()
    }


    fn new(bus: u8, device_slot: u8, function: u8) -> Self {
        Self {
            bus,
            device_slot,
            function,
        }
    }


    fn config_addr_at(&self, offset: u8) -> ConfigAddrRegister {
        ConfigAddrRegister::new(offset, self.function, self.device_slot, self.bus)
    }
}


impl CommonHeaderHoldable for ConfigurationSpace {
    fn as_config_space(&self) -> &ConfigurationSpace {
        self
    }
}


fn select_single_function_device(config_space: ConfigurationSpace) -> Function {
    let device_header = if (config_space.class_code() == ClassCode::BridgeDevice)
        && (config_space.sub_class()) == Subclass::PciToPciBridge
    {
        SingleFunctionDevice::PciToPciBride(PciToPciBridgeHeader::new(config_space))
    } else {
        SingleFunctionDevice::General(GeneralHeader::new(config_space))
    };

    Function::Single(device_header)
}


#[cfg(test)]
mod tests {
    use crate::configuration_space::ConfigurationSpace;

    #[test]
    fn it_new_first_offset() {
        let p = ConfigurationSpace::new(1, 2, 3).config_addr_at(0);
        let inner = p.as_data();

        assert_eq!((inner >> 31), 1);

        assert_eq!(p.bus(), 1);
        assert_eq!(((inner >> 16) & 0xFF), 1);

        assert_eq!(p.device_slot(), 2);
        assert_eq!(((inner >> 11) & 0b11111), 2);

        assert_eq!(p.function(), 3);
        assert_eq!(((inner >> 8) & 0b111), 3);

        assert_eq!(p.register_offset(), 0);
        assert_eq!((inner & 0xFC), 0);
    }
}
