#[derive(Debug)]
pub struct EndpointId(usize);

impl EndpointId {
    pub fn from_addr(addr: usize) -> Self {
        Self(addr)
    }

    pub fn from_endpoint_num(ep_num: usize, dir_in: bool) -> Self {
        Self(ep_num << 1 | if dir_in { 1 } else { 0 })
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
