use crate::configuration_space::msi::msi_capability_register::access::control::ControlAccessor;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::control::Control;
use crate::configuration_space::ConfigurationSpace;
use crate::error::OldPciResult;
use kernel_lib::io::io_memory_accessible::IoMemoryAccessible;

#[derive(Debug)]
pub struct MsiXCapabilityRegisters<Io>
where
    Io: IoMemoryAccessible,
{
    msi_cap_addr: u8,
    configuration_space: ConfigurationSpace,
    control: ControlAccessor,
    io: Io,
}

impl<Io> MsiXCapabilityRegisters<Io>
where
    Io: IoMemoryAccessible,
{
    pub fn new(
        msi_cap_addr: u8,
        configuration_space: ConfigurationSpace,
        io: Io,
    ) -> OldPciResult<MsiXCapabilityRegisters<Io>> {
        let control = ControlAccessor::new();


        Ok(Self {
            msi_cap_addr,
            configuration_space,
            control,
            io,
        })
    }


    pub fn read_control_register(&mut self) -> OldPciResult<Control> {
        self.control
            .read(&mut self.io, &self.configuration_space, self.msi_cap_addr)
    }
}
