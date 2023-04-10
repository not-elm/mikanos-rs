use crate::configuration_space::io::config_address_register::ConfigAddrRegister;
use crate::configuration_space::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::from_u32::TryFromU32;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::MessageData;
use crate::configuration_space::ConfigurationSpace;
use crate::error::PciResult;

#[derive(Debug, Clone)]
pub struct MessageDataAccessor {}


impl MessageDataAccessor {
    pub const fn new() -> Self {
        Self {}
    }
}


impl<Io> MsiCapabilityAccessible<Io, MessageData> for MessageDataAccessor
where
    Io: IoMemoryAccessible,
{
    fn read(
        &self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
    ) -> PciResult<MessageData> {
        io.write_config_addr(config_addr_register(configuration_space, msi_cap_addr));
        let raw_data = io.read_config_data();
        MessageData::try_from_u32(raw_data)
    }

    fn write(
        &mut self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
        register: MessageData,
    ) {
        io.write_config_addr(config_addr_register(configuration_space, msi_cap_addr));
        io.write_config_data(register.raw());
    }
}

fn config_addr_register(
    configuration_space: &ConfigurationSpace,
    msi_cap_addr: u8,
) -> ConfigAddrRegister {
    let offset = msi_cap_addr + 0x0C;
    ConfigAddrRegister::new(
        offset,
        configuration_space.function,
        configuration_space.device_slot,
        configuration_space.bus,
    )
}
