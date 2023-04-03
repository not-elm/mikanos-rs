use core::fmt::Debug;

use crate::error::PciResult;
use crate::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::registers::traits::capability_registers_accessible::CapabilityRegistersAccessible;
use crate::xhc::registers::traits::config_register_accessible::ConfigRegisterAccessible;
use crate::xhc::registers::traits::device_context_bae_address_array_pointer_accessible::DeviceContextBaseAddressArrayPointerAccessible;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::registers::traits::registers_operation::RegistersOperation;
use crate::xhc::registers::traits::usb_command_register_accessible::UsbCommandRegisterAccessible;

pub struct External<M>(
    xhci::registers::Registers<M>,
    xhci::extended_capabilities::List<M>,
)
where
    M: xhci::accessor::Mapper + Clone;

impl<M> External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    pub fn new(mmio_addr: MemoryMappedAddr, mapper: M) -> Self {
        let registers = unsafe { xhci::Registers::new(mmio_addr.addr(), mapper.clone()) };
        let e = unsafe {
            xhci::extended_capabilities::List::new(
                mmio_addr.addr(),
                registers.capability.hccparams1.read_volatile(),
                mapper.clone(),
            )
            .unwrap()
        };
        Self(registers, e)
    }
}

impl<M> External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn registers_mut(&mut self) -> &mut xhci::registers::Registers<M> {
        &mut self.0
    }
}

impl<M> RegistersOperation for External<M>
where
    M: xhci::accessor::Mapper + Clone + Debug,
{
    fn reset(&mut self) -> PciResult {
        let registers = self.registers_mut();
        registers.operational.usbcmd.update_volatile(|usb_cmd| {
            usb_cmd.clear_interrupter_enable();
            usb_cmd.clear_host_system_error_enable();
            usb_cmd.clear_enable_wrap_event();
        });

        while !registers.operational.usbsts.read_volatile().hc_halted() {}
        registers.operational.usbcmd.update_volatile(|usb_cmd| {
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
        self.0.operational.usbcmd.update_volatile(|u| {
            u.set_interrupter_enable();
        });
        self.0
            .interrupter_register_set
            .interrupter_mut(0)
            .imod
            .update_volatile(|moderation| {
                moderation.set_interrupt_moderation_interval(4000);
            });
        self.0.operational.usbcmd.update_volatile(|u| {
            u.set_run_stop();
        });

        while self.0.operational.usbsts.read_volatile().hc_halted() {}

        let connect_index = self
            .0
            .port_register_set
            .into_iter()
            .position(|p| p.portsc.current_connect_status())
            .unwrap();

        self.0
            .port_register_set
            .update_volatile_at(connect_index, |p| {
                p.portsc.set_port_reset();
                p.portsc.set_wake_on_connect_enable();
            });

        while self
            .0
            .port_register_set
            .read_volatile_at(connect_index)
            .portsc
            .port_reset()
        {}

        Ok(())
    }
}

impl<M> CapabilityRegistersAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn read_max_scratchpad_buffers_len(&self) -> usize {
        self.0
            .capability
            .hcsparams2
            .read_volatile()
            .max_scratchpad_buffers() as usize
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

impl<M> UsbCommandRegisterAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn write_command_ring_addr(&mut self, command_ring_addr: u64) -> PciResult {
        let registers = self.registers_mut();

        registers.operational.crcr.update_volatile(|crcr| {
            crcr.set_ring_cycle_state();

            crcr.set_command_ring_pointer(command_ring_addr);
        });

        Ok(())
    }
}

impl<M> DeviceContextBaseAddressArrayPointerAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) -> PciResult {
        self.registers_mut()
            .operational
            .dcbaap
            .update_volatile(|device_context| device_context.set(device_context_addr));

        Ok(())
    }
}

impl<M> InterrupterSetRegisterAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn write_event_ring_dequeue_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_addr: u64,
    ) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erdp
            .update_volatile(|erdp| erdp.set_event_ring_dequeue_pointer(event_ring_segment_addr));

        Ok(())
    }

    fn write_event_ring_segment_table_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_table_addr: u64,
    ) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erstba
            .update_volatile(|erstba| erstba.set(event_ring_segment_table_addr));

        Ok(())
    }

    fn write_interrupter_enable_at(&mut self, index: usize, is_enable: bool) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                if is_enable {
                    iman.set_interrupt_enable();
                } else {
                    iman.clear_interrupt_enable();
                }
            });

        Ok(())
    }

    fn write_interrupter_pending_at(&mut self, index: usize, is_pending: bool) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .iman
            .update_volatile(|iman| {
                if is_pending {
                    iman.set_0_interrupt_pending();
                } else {
                    iman.clear_interrupt_pending();
                }
            });

        Ok(())
    }

    fn read_dequeue_pointer_addr_at(&self, index: usize) -> u64 {
        self.0
            .interrupter_register_set
            .interrupter(index)
            .erdp
            .read_volatile()
            .event_ring_dequeue_pointer()
    }

    fn write_event_ring_segment_table_size(&mut self, index: usize, size: u16) -> PciResult {
        self.registers_mut()
            .interrupter_register_set
            .interrupter_mut(index)
            .erstsz
            .update_volatile(|e| e.set(size));

        Ok(())
    }
}

impl<M> DoorbellRegistersAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn notify_at(&mut self, index: usize, target: u8, stream_id: u16) -> PciResult {
        self.registers_mut()
            .doorbell
            .update_volatile_at(index, |doorbell| {
                doorbell.set_doorbell_target(target);
                doorbell.set_doorbell_stream_id(stream_id);
            });

        Ok(())
    }
}

impl<M> PortRegistersAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn reset_port_at(&mut self, port_id: u8) -> PciResult {
        self.registers_mut()
            .port_register_set
            .update_volatile_at(port_index(port_id), |port| {
                port.portsc.set_port_reset();
                port.portsc.set_wake_on_connect_enable();
            });
        while self
            .0
            .port_register_set
            .read_volatile_at(port_index(port_id))
            .portsc
            .port_reset()
        {}
        Ok(())
    }

    fn read_port_speed_at(&self, port_id: u8) -> PciResult<u8> {
        Ok(self
            .0
            .port_register_set
            .read_volatile_at(port_index(port_id))
            .portsc
            .port_speed())
    }

    fn read_port_reset_change_status(&self, port_id: u8) -> PciResult<bool> {
        Ok(self
            .0
            .port_register_set
            .read_volatile_at(port_index(port_id))
            .portsc
            .port_reset_change())
    }

    fn clear_port_reset_change_at(&mut self, port_id: u8) -> PciResult {
        self.registers_mut()
            .port_register_set
            .update_volatile_at(port_index(port_id), |port| {
                port.portsc.clear_port_reset_change();
            });

        Ok(())
    }
}

fn port_index(port_id: u8) -> usize {
    (port_id - 1) as usize
}
