use core::fmt::Debug;
use core::num::NonZeroUsize;

use crate::error::PciResult;
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::registers::traits::config_register_accessible::ConfigRegisterAccessible;
use crate::xhc::registers::traits::registers_operation::RegistersOperation;
use crate::xhc::registers::XhcRegisters;

mod capability;
mod device_context_base_address;
mod doorbell;
mod interrupter;
mod port;
mod usb_command;

pub struct External<M>(xhci::registers::Registers<M>)
    where
        M: xhci::accessor::Mapper + Clone;


impl<M> External<M>
    where
        M: xhci::accessor::Mapper + Clone,
{
    pub fn new(mmio_addr: MemoryMappedAddr, mapper: M) -> Self {
        let registers = unsafe { xhci::Registers::new(mmio_addr.addr(), mapper.clone()) };
        Self(registers)
    }


    fn registers_mut(&mut self) -> &mut xhci::registers::Registers<M> {
        &mut self.0
    }
}


impl<M> XhcRegisters for External<M> where M: xhci::accessor::Mapper + Clone + Debug {}


impl<M> RegistersOperation for External<M>
    where
        M: xhci::accessor::Mapper + Clone + Debug,
{
    fn reset(&mut self) -> PciResult {
        let registers = self.registers_mut();
        registers
            .operational
            .usbcmd
            .update_volatile(|usb_cmd| {
                usb_cmd.clear_run_stop();
            });

        while !registers
            .operational
            .usbsts
            .read_volatile()
            .hc_halted()
        {}
        registers
            .operational
            .usbcmd
            .update_volatile(|usb_cmd| {
                usb_cmd.set_host_controller_reset();
            });
        while registers
            .operational
            .usbsts
            .read_volatile()
            .controller_not_ready()
        {}

        Ok(())
    }


    fn run(&mut self) -> PciResult {
        self.0
            .operational
            .usbcmd
            .update_volatile(|u| {
                u.set_interrupter_enable();
            });

        self.0
            .interrupter_register_set
            .interrupter_mut(0)
            .imod
            .update_volatile(|u| {
                u.set_interrupt_moderation_interval(100);
            });
        self.0
            .operational
            .usbcmd
            .update_volatile(|u| {
                u.set_run_stop();
            });

        while self
            .0
            .operational
            .usbsts
            .read_volatile()
            .hc_halted()
        {}


        Ok(())
    }
}


impl<M> ConfigRegisterAccessible for External<M>
    where
        M: xhci::accessor::Mapper + Clone,
{
    fn write_max_device_slots_enabled(&mut self, device_slots: u8) -> PciResult {
        self.registers_mut()
            .operational
            .config
            .update_volatile(|config| {
                config.set_max_device_slots_enabled(device_slots);
            });

        Ok(())
    }
}


#[derive(Clone, Debug, Default)]
pub struct IdentityMapper;

impl xhci::accessor::Mapper for IdentityMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> NonZeroUsize {
        NonZeroUsize::new_unchecked(phys_start)
    }

    fn unmap(&mut self, _virtual_start: usize, _bytes: usize) {}
}
