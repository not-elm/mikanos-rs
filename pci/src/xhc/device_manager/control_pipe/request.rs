use crate::xhc::device_manager::device_slot::RequestType;
use xhci::ring::trb::transfer::SetupStage;

pub enum Request {
    GetDescriptor(SetupStage),
}

impl Request {
    pub fn get_descriptor(desc_type: u16, desc_index: u16, len: u16) -> Self {
        Self::GetDescriptor(get_descriptor(desc_type, desc_index, len))
    }

    pub fn into_setup_stage(self) -> SetupStage {
        match self {
            Self::GetDescriptor(setup) => setup,
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
