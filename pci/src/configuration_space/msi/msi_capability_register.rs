use core::cmp::min;
use core::fmt::{Debug, Formatter};

use kernel_lib::serial_println;

use crate::configuration_space::ConfigurationSpace;
use crate::configuration_space::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::msi::msi_capability_register::access::control::ControlAccessor;
use crate::configuration_space::msi::msi_capability_register::access::message_address::MessageAddressAccessor;
use crate::configuration_space::msi::msi_capability_register::access::message_data::MessageDataAccessor;
use crate::configuration_space::msi::msi_capability_register::access::msi_capability_accessible::MsiCapabilityAccessible;
use crate::configuration_space::msi::msi_capability_register::structs::control::Control;
use crate::configuration_space::msi::msi_capability_register::structs::message_address::MessageAddress;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::delivery_mode::DeliveryMode;
use kernel_lib::interrupt::interrupt_vector::InterruptVector;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::level_for_trigger_mode::LevelForTriggerMode;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::MessageData;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::trigger_mode::TriggerMode;
use crate::error::PciResult;

pub mod access;
pub mod structs;

#[derive(Clone)]
pub struct MsiCapabilityRegister<Io>
where
    Io: IoMemoryAccessible,
{
    control: ControlAccessor,
    message_address: MessageAddressAccessor,
    message_data: MessageDataAccessor,
    msi_cap_addr: u8,
    configuration_space: ConfigurationSpace,
    io: Io,
}

impl<Io> Debug for MsiCapabilityRegister<Io>
where
    Io: IoMemoryAccessible + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MsiCapabilityRegister")
            .field(
                "Control",
                &self
                    .clone()
                    .read_control_register()
                    .unwrap(),
            )
            .field(
                "MessageAddress",
                &self
                    .clone()
                    .read_message_address_register()
                    .unwrap(),
            )
            .field(
                "MessageData",
                &self
                    .clone()
                    .read_message_data_register()
                    .unwrap(),
            )
            .finish()
    }
}

impl<Io> MsiCapabilityRegister<Io>
where
    Io: IoMemoryAccessible,
{
    pub fn new(
        msi_cap_addr: u8,
        configuration_space: ConfigurationSpace,
        mut io: Io,
    ) -> PciResult<MsiCapabilityRegister<Io>> {
        let control = ControlAccessor::new();
        let message_address = MessageAddressAccessor::new(
            control
                .read(&mut io, &configuration_space, msi_cap_addr)?
                .is_64bit_addr_capable(),
        );

        Ok(Self {
            control,
            message_address,
            message_data: MessageDataAccessor::new(),
            msi_cap_addr,
            configuration_space,
            io,
        })
    }


    pub fn read_control_register(&mut self) -> PciResult<Control> {
        self.control
            .read(&mut self.io, &self.configuration_space, self.msi_cap_addr)
    }


    pub fn read_message_address_register(&mut self) -> PciResult<MessageAddress> {
        self.message_address
            .read(&mut self.io, &self.configuration_space, self.msi_cap_addr)
    }


    pub fn read_message_data_register(&mut self) -> PciResult<MessageData> {
        self.message_data
            .read(&mut self.io, &self.configuration_space, self.msi_cap_addr)
    }

    pub fn enable(
        &mut self,
        apic_id: u8,
        trigger_mode: TriggerMode,
        vector: InterruptVector,
        delivery_mode: DeliveryMode,
        multiple_msg_enable: u8,
    ) -> PciResult {
        self.control.update(
            &mut self.io,
            &self.configuration_space,
            self.msi_cap_addr,
            |control| {
                serial_println!("CONTROL {:?}", control);
                control.set_msi_enable();

                let capable = control.multiple_msg_capable();
                control.set_multiple_msg_enable(min(capable, multiple_msg_enable));
            },
        )?;
        self.message_address.update(
            &mut self.io,
            &self.configuration_space,
            self.msi_cap_addr,
            |message_addr| {
                message_addr.set_message_addr(0xfee00000 | ((apic_id as usize) << 12));
            },
        )?;

        self.message_data.update(
            &mut self.io,
            &self.configuration_space,
            self.msi_cap_addr,
            |message_data| {
                message_data.set_vector(vector);
                message_data.set_delivery_mode(delivery_mode);
                if trigger_mode.is_level() {
                    message_data.set_trigger_mode(TriggerMode::Level);
                    message_data.set_level_for_trigger_mode(LevelForTriggerMode::Assert);
                }
            },
        )?;
        Ok(())
    }
}
