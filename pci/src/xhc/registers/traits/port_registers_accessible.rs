use crate::error::OldPciResult;

pub trait PortRegistersAccessible {
    fn reset_port_at(&mut self, port_id: u8) -> OldPciResult;
    fn read_port_speed_at(&self, port_id: u8) -> OldPciResult<u8>;
    fn read_port_reset_change_status(&self, port_id: u8) -> OldPciResult<bool>;
    fn clear_port_reset_change_at(&mut self, port_id: u8) -> OldPciResult;
    fn reset_all(&mut self);
}
