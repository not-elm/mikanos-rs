use crate::error::PciResult;

pub trait PortRegistersAccessible {
    fn reset_port_at(&mut self, port_id: u8) -> PciResult;
    fn read_port_speed_at(&self, port_id: u8) -> PciResult<u8>;
    fn read_port_reset_change_status(&self, port_id: u8) -> PciResult<bool>;
    fn clear_port_reset_change_at(&mut self, port_id: u8) -> PciResult;
}
