use crate::configuration_space::io::config_address_register::ConfigAddrRegister;
use crate::configuration_space::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::control::Control;
use crate::configuration_space::msi::msi_capability_register::structs::from_u32::TryFromU32;
use crate::configuration_space::ConfigurationSpace;
use crate::error::PciResult;

#[derive(Debug, Clone)]
pub struct ControlAccessor {}



impl ControlAccessor {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<Io> MsiCapabilityAccessible<Io, Control> for ControlAccessor
where
    Io: IoMemoryAccessible,
{
    fn read(
        &self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
    ) -> PciResult<Control> {
        let raw = io.read_config_data_with_set_addr(config_addr_register_control(
            configuration_space,
            msi_cap_addr,
        ));
        Control::try_from_u32(raw)
    }


    fn write(
        &mut self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
        register: Control,
    ) {
        io.write_config_data_with_set_addr(
            config_addr_register_control(configuration_space, msi_cap_addr),
            register.raw(),
        )
    }
}


fn config_addr_register_control(
    configuration_space: &ConfigurationSpace,
    msi_cap_addr: u8,
) -> ConfigAddrRegister {
    ConfigAddrRegister::new(
        msi_cap_addr,
        configuration_space.function,
        configuration_space.device_slot,
        configuration_space.bus,
    )
}
