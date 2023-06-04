use crate::configuration_space::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::ConfigurationSpace;
use crate::error::OldPciResult;

pub trait MsiCapabilityAccessible<Io, Register>
where
    Io: IoMemoryAccessible,
{
    fn read(
        &self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
    ) -> OldPciResult<Register>;


    fn write(
        &mut self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
        register: Register,
    );


    fn update(
        &mut self,
        io: &mut Io,
        configuration_space: &ConfigurationSpace,
        msi_cap_addr: u8,
        fun: impl Fn(&mut Register),
    ) -> OldPciResult {
        let mut register = self.read(io, configuration_space, msi_cap_addr)?;
        fun(&mut register);
        self.write(io, configuration_space, msi_cap_addr, register);
        Ok(())
    }
}
