use xhci::context::{EndpointHandler, EndpointState, EndpointType};

use crate::xhc::device_manager::descriptor::structs::endpoint_descriptor::EndpointDescriptor;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::device_manager::endpoint_id::EndpointId;

#[derive(Debug, Clone)]
pub struct EndpointConfig {
    ep_id: EndpointId,
    ep_type: EndpointType,
    max_packet_size: u16,
    interval: u8,
}

impl EndpointConfig {
    pub fn new(endpoint: &EndpointDescriptor) -> Self {
        Self {
            ep_id: EndpointId::from_endpoint_num(
                endpoint.endpoint_address.number() as usize,
                endpoint.endpoint_address.dir_in(),
            ),
            ep_type: to_endpoint_type(endpoint.attributes.transfer_type()),
            max_packet_size: endpoint.max_packet_size,
            interval: endpoint.interval,
        }
    }
    pub fn endpoint_id(&self) -> EndpointId {
        self.ep_id
    }
    pub fn endpoint_type(&self) -> EndpointType {
        self.ep_type
    }
    pub fn max_packet_size(&self) -> u16 {
        self.max_packet_size
    }
    pub fn interval(&self) -> u8 {
        self.interval
    }

    pub fn device_context_index(&self) -> DeviceContextIndex {
        DeviceContextIndex::from_endpoint_id(self.ep_id)
    }

    pub fn write_endpoint_context(
        &self,
        tr_buff_addr: u64,
        endpoint_ctx: &mut dyn EndpointHandler,
    ) {
        endpoint_ctx.set_max_packet_size(self.max_packet_size);
        endpoint_ctx.set_interval(self.interval - 1);
        endpoint_ctx.set_average_trb_length(1);
        endpoint_ctx.set_endpoint_state(EndpointState::Running);
        endpoint_ctx.set_error_count(3);
        endpoint_ctx.set_tr_dequeue_pointer(tr_buff_addr);
        endpoint_ctx.set_endpoint_type(EndpointType::InterruptIn);
        endpoint_ctx.set_dequeue_cycle_state();
    }
}

fn to_endpoint_type(v: u8) -> EndpointType {
    match v {
        0 => EndpointType::NotValid,
        1 => EndpointType::IsochOut,
        2 => EndpointType::BulkOut,
        3 => EndpointType::InterruptOut,
        4 => EndpointType::Control,
        5 => EndpointType::IsochIn,
        6 => EndpointType::BulkIn,
        7 => EndpointType::InterruptIn,
        _ => EndpointType::NotValid,
    }
}
