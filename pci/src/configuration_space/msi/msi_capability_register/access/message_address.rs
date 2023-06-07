use kernel_lib::io::config_address_register::ConfigAddrRegister;
use kernel_lib::io::io_memory_accessible::IoMemoryAccessible;

use crate::configuration_space::ConfigurationSpace;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::message_address::MessageAddress;
use crate::error::PciResult;

#[derive(Debug, Clone)]
pub struct MessageAddressAccessor {
    is_64bit_address: bool,
}


impl MessageAddressAccessor {
    pub const fn new(is_64bit_address: bool) -> Self {
        Self { is_64bit_address }
    }
}


impl<Io> MsiCapabilityAccessible<Io, MessageAddress> for MessageAddressAccessor
    where
        Io: IoMemoryAccessible,
{
    fn read(
        &self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
    ) -> PciResult<MessageAddress> {
        let msi_lower_addr = io.read_config_data_with_set_addr(config_addr_offset_lower(
            configuration_space,
            msi_cap_addr,
        ));

        let msi_upper_addr = io.read_config_data_with_set_addr(config_addr_offset_upper(
            configuration_space,
            msi_cap_addr,
        ));

        Ok(MessageAddress::new(msi_lower_addr, msi_upper_addr))
    }


    fn write(
        &mut self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
        register: MessageAddress,
    ) {
        io.write_config_data_with_set_addr(
            config_addr_offset_lower(configuration_space, msi_cap_addr),
            register.message_lower_addr(),
        );

        if self.is_64bit_address {
            io.write_config_data_with_set_addr(
                config_addr_offset_upper(configuration_space, msi_cap_addr),
                register.message_upper_addr(),
            );
        }
    }
}

fn config_addr_offset_lower(
    configuration_space: &ConfigurationSpace,
    msi_cap_addr: u8,
) -> ConfigAddrRegister {
    config_addr_register(configuration_space, msi_cap_addr, 0x04)
}


fn config_addr_offset_upper(
    configuration_space: &ConfigurationSpace,
    msi_cap_addr: u8,
) -> ConfigAddrRegister {
    config_addr_register(configuration_space, msi_cap_addr, 0x08)
}


fn config_addr_register(
    configuration_space: &ConfigurationSpace,
    msi_cap_addr: u8,
    offset: u8,
) -> ConfigAddrRegister {
    ConfigAddrRegister::new(
        msi_cap_addr + offset,
        configuration_space.function,
        configuration_space.device_slot,
        configuration_space.bus,
    )
}
