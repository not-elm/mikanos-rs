use kernel_lib::io::io_memory_accessible::IoMemoryAccessible;

use crate::configuration_space::ConfigurationSpace;
use crate::configuration_space::msi::msi_capability_register::access::control::ControlAccessor;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::control::Control;
use crate::error::PciResult;

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
    ) -> PciResult<MsiXCapabilityRegisters<Io>> {
        let control = ControlAccessor::new();


        Ok(Self {
            msi_cap_addr,
            configuration_space,
            control,
            io,
        })
    }


    pub fn read_control_register(&mut self) -> PciResult<Control> {
        self.control
            .read(&mut self.io, &self.configuration_space, self.msi_cap_addr)
    }
}
