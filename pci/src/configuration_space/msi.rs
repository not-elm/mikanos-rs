use core::fmt::Debug;

use crate::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::configuration_space::device::header_type::general_header::GeneralHeader;
use crate::configuration_space::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::msi::msi_capability_register::access::control::ControlAccessor;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::capability_id::CapabilityId;
use crate::configuration_space::msi::msi_capability_register::MsiCapabilityRegister;
use crate::configuration_space::msi::msi_x::MsiXCapabilityRegisters;
use crate::error::OldPciResult;

pub mod msi_capability_register;
pub mod msi_x;

#[derive(Debug)]
pub struct InterruptCapabilityRegisterIter<Io>
where
    Io: IoMemoryAccessible + Clone,
{
    general_header: GeneralHeader,
    msi_cap_addr: u8,
    io: Io,
}

impl<Io> InterruptCapabilityRegisterIter<Io>
where
    Io: IoMemoryAccessible + Clone + Debug,
{
    pub fn new(general_header: GeneralHeader, io: Io) -> InterruptCapabilityRegisterIter<Io> {
        let msi_cap_addr = general_header.msi_capability_pointer();
        Self {
            general_header,
            msi_cap_addr,
            io,
        }
    }
}


impl<Io> Iterator for InterruptCapabilityRegisterIter<Io>
where
    Io: IoMemoryAccessible + Debug + Clone,
{
    type Item = OldPciResult<InterruptCapabilityRegister<Io>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.msi_cap_addr == 0 {
            return None;
        }
        let mut register = InterruptCapabilityRegister::new(
            self.general_header.clone(),
            self.msi_cap_addr,
            self.io.clone(),
        );

        self.msi_cap_addr = register
            .as_mut()
            .map_or(0, |r| {
                r.next_msi_cap_addr()
                    .unwrap_or(0)
            });

        Some(register)
    }
}

#[derive(Debug)]
pub enum InterruptCapabilityRegister<Io>
where
    Io: IoMemoryAccessible + Clone,
{
    Msi(MsiCapabilityRegister<Io>),
    MsiX(MsiXCapabilityRegisters<Io>),
}


impl<Io> InterruptCapabilityRegister<Io>
where
    Io: IoMemoryAccessible + Debug + Clone,
{
    pub fn new(
        general_header: GeneralHeader,
        msi_cap_addr: u8,
        mut io: Io,
    ) -> OldPciResult<InterruptCapabilityRegister<Io>> {
        let configuration_space = general_header
            .as_config_space()
            .clone();
        let capability_id = ControlAccessor::new()
            .read(&mut io, &configuration_space, msi_cap_addr)?
            .capability_id();

        match capability_id {
            CapabilityId::Msi => Ok(Self::Msi(MsiCapabilityRegister::new(
                msi_cap_addr,
                configuration_space,
                io,
            )?)),
            CapabilityId::MsiX => Ok(Self::MsiX(MsiXCapabilityRegisters::new(
                msi_cap_addr,
                configuration_space,
                io,
            )?)),
        }
    }

    pub fn next_msi_cap_addr(&mut self) -> OldPciResult<u8> {
        Ok(match self {
            Self::Msi(msi) => msi
                .read_control_register()?
                .next_cap_ptr(),

            Self::MsiX(msi_x) => msi_x
                .read_control_register()?
                .next_cap_ptr(),
        })
    }

    pub fn msi(self) -> Option<MsiCapabilityRegister<Io>> {
        match self {
            Self::Msi(msi) => Some(msi),
            _ => None,
        }
    }
}
