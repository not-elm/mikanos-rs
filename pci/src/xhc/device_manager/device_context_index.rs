use crate::flag_to_num;
use crate::xhc::device_manager::endpoint_id::EndpointId;
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DeviceContextIndex(usize);

impl DeviceContextIndex {
    pub fn from_endpoint_id(endpoint_id: EndpointId) -> Self {
        Self(endpoint_id.value())
    }
    pub fn from_dci(dci: usize) -> Self {
        Self(dci)
    }
    pub fn new(endpoint_num: usize, is_control_in: bool) -> Self {
        Self(
            2 * endpoint_num
                + (if endpoint_num == 0 {
                    1
                } else {
                    flag_to_num(is_control_in)
                }),
        )
    }
    pub fn as_u8(&self) -> u8 {
        self.0 as u8
    }
    pub fn value(&self) -> usize {
        self.0
    }
}
