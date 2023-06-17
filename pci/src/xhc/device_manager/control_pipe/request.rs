use xhci::ring::trb::transfer::{SetupStage, TransferType};

use crate::xhc::device_manager::control_pipe::request::Request::{
    Configuration, GetDescriptor, GetReport, SetProtocol,
};
use crate::xhc::device_manager::control_pipe::request_type::RequestType;

pub enum Request {
    GetDescriptor(SetupStage),
    Configuration(SetupStage),
    SetProtocol(SetupStage),
    GetReport(SetupStage),
}

impl Request {
    pub fn get_descriptor(desc_type: u16, desc_index: u16, len: u16) -> Self {
        GetDescriptor(get_descriptor(desc_type, desc_index, len))
    }
    pub fn get_report(report_len: u16, interface_number: u16) -> Self {
        let mut setup_data = SetupStage::new();
        const GET_REPORT: u8 = 1;
        setup_data.set_request_type(
            RequestType::new()
                .with_direction(true)
                .with_ty(1)
                .with_recipient(1)
                .raw(),
        );
        setup_data.set_request(GET_REPORT);
        setup_data.set_value(0x100);
        setup_data.set_index(interface_number);
        setup_data.set_length(report_len);
        setup_data.set_transfer_type(TransferType::In);
        GetReport(setup_data)
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


    pub fn set_protocol(request_type: RequestType, interface_num: u16) -> Self {
        let mut setup = SetupStage::new();

        setup.set_interrupt_on_completion();
        setup.set_index(interface_num);
        setup.set_value(0);
        setup.set_request_type(request_type.raw());
        setup.set_request(11);
        setup.set_index(0);
        SetProtocol(setup)
    }


    pub fn setup_stage(&self) -> SetupStage {
        match self {
            GetDescriptor(setup) => *setup,
            Configuration(setup) => *setup,
            SetProtocol(setup) => *setup,
            GetReport(setup) => *setup,
        }
    }
}

fn get_descriptor(desc_type: u16, desc_index: u16, len: u16) -> SetupStage {
    let mut setup_data = SetupStage::new();
    setup_data.set_request_type(
        RequestType::new()
            .with_direction(true)
            .into(),
    );
    // GET_DESCRIPTOR
    setup_data.set_request(6);
    setup_data.set_value(desc_type << 8 | desc_index);
    setup_data.set_index(0);
    setup_data.set_length(len);
    setup_data
}
