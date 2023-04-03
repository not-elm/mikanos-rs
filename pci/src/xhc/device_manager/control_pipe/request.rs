use xhci::ring::trb::transfer::SetupStage;

use crate::xhc::device_manager::control_pipe::request::Request::{
    Configuration, GetDescriptor, SetProtocol,
};
use crate::xhc::device_manager::control_pipe::request_type::RequestType;

pub enum Request {
    GetDescriptor(SetupStage),
    Configuration(SetupStage),
    SetProtocol(SetupStage),
}

impl Request {
    pub fn get_descriptor(desc_type: u16, desc_index: u16, len: u16) -> Self {
        GetDescriptor(get_descriptor(desc_type, desc_index, len))
    }
    pub fn configuration(config_value: u16) -> Self {
        let mut setup_data = SetupStage::new();
        const CONFIGURATION: u8 = 9;
        setup_data.set_request(CONFIGURATION);
        setup_data.set_value(config_value);
        setup_data.set_index(0);
        setup_data.set_length(0);
        Configuration(setup_data)
    }
    pub fn set_protocol(request_type: RequestType) -> Self {
        let mut setup = SetupStage::new();

        setup.set_value(0);
        setup.set_request_type(request_type.raw());
        setup.set_request(11);
        setup.set_index(0);
        SetProtocol(setup)
    }
    pub fn into_setup_stage(self) -> SetupStage {
        match self {
            GetDescriptor(setup) => setup,
            Configuration(setup) => setup,
            SetProtocol(setup) => setup,
        }
    }
}

fn get_descriptor(desc_type: u16, desc_index: u16, len: u16) -> SetupStage {
    let mut setup_data = SetupStage::new();
    setup_data.set_request_type(RequestType::new().with_direction(true).into());
    // GET_DESCRIPTOR
    setup_data.set_request(6);
    setup_data.set_value(desc_type << 8 | desc_index);
    setup_data.set_index(0);
    setup_data.set_length(len);
    setup_data
}
