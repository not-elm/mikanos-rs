#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EndpointId(usize);

impl EndpointId {
    pub fn from_addr(addr: usize) -> Self {
        Self(addr)
    }

    pub fn from_endpoint_num(ep_num: usize, dir_in: bool) -> Self {
        Self((2 * ep_num) | if dir_in { 1 } else { 0 })
    }
    pub fn is_control_in(&self) -> bool {
        self.0 & 0b1 == 1
    }
    pub fn value(&self) -> usize {
        self.0
    }
}
