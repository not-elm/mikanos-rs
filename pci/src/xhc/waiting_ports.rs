use alloc::vec::Vec;

#[derive(Default)]
pub struct WaitingPorts {
    waiting_ports: Vec<u8>,
}


impl WaitingPorts {
    pub fn push(&mut self, port_id: u8) {
        self.waiting_ports
            .push(port_id);
    }


    pub fn pop(&mut self) -> Option<u8> {
        self.waiting_ports.pop()
    }
}
