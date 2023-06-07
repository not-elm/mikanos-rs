use crate::error::PciResult;
use crate::xhc::registers::external::External;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;

impl<M> PortRegistersAccessible for External<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn reset_port_at(&mut self, port_id: u8) -> PciResult {
        self.registers_mut()
            .port_register_set
            .update_volatile_at(port_index(port_id), |port| {
                port.portsc.set_port_reset();
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
                port.portsc
                    .set_0_port_reset_change();
            });

        Ok(())
    }


    fn reset_all(&mut self) {
        let connect_index = self
            .0
            .port_register_set
            .into_iter()
            .position(|p| {
                p.portsc
                    .current_connect_status()
            })
            .unwrap();

        self.0
            .port_register_set
            .update_volatile_at(connect_index, |p| {
                p.portsc.set_port_reset();
            });

        while self
            .0
            .port_register_set
            .read_volatile_at(connect_index)
            .portsc
            .port_reset()
        {}
    }
}

fn port_index(port_id: u8) -> usize {
    (port_id - 1) as usize
}
