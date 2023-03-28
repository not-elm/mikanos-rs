use crate::error::PciResult;

pub trait PortRegistersAccessible {
    fn reset_port_at(&mut self, port_id: u8) -> PciResult;
    fn clear_port_reset_change_at(&mut self, port_id: u8) -> PciResult;
}
