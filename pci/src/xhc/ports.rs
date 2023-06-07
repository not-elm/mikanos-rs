use alloc::vec::Vec;

#[derive(Default)]
pub struct Ports {
    addressing_ports: Vec<u8>,
}


impl Ports {
    pub fn push_waiting_port(&mut self, port_id: u8) {
        self.addressing_ports
            .push(port_id);
    }


    pub fn pop_waiting_port(&mut self) -> Option<u8> {
        self.addressing_ports.pop()
    }
}
