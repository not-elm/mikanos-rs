use crate::error::{DeviceReason, PciError, PciResult};
use crate::xhc::transfer::trb_raw_data::TrbRawData;
use xhci::ring::trb::transfer::{DataStage, Normal, SetupStage, StatusStage};

#[derive(Debug)]
pub enum TargetEvent {
    Normal(Normal),
    Setup(SetupStage),
    DataStage(DataStage),
    StatusStage(StatusStage),
}

impl TargetEvent {
    pub fn new(target_pointer_addr: u64) -> Option<Self> {
        let raw_data_data = TrbRawData::from_addr(target_pointer_addr);
        match raw_data_data.template().trb_type() {
            1 => Some(TargetEvent::Normal(
                Normal::try_from(raw_data_data.into_u32_array()).ok()?,
            )),
            2 => Some(TargetEvent::Setup(
                SetupStage::try_from(raw_data_data.into_u32_array()).ok()?,
            )),
            3 => Some(TargetEvent::DataStage(
                DataStage::try_from(raw_data_data.into_u32_array()).ok()?,
            )),
            4 => Some(TargetEvent::StatusStage(
                StatusStage::try_from(raw_data_data.into_u32_array()).ok()?,
            )),
            _ => None,
        }
    }

    pub fn data_stage(self) -> PciResult<DataStage> {
        if let TargetEvent::DataStage(data_stage) = self {
            Ok(data_stage)
        } else {
            Err(PciError::FailedOperateDevice(
                DeviceReason::InvalidTargetEvent,
            ))
        }
    }

    pub fn status_stage(self) -> PciResult<StatusStage> {
        if let TargetEvent::StatusStage(status_stage) = self {
            Ok(status_stage)
        } else {
            Err(PciError::FailedOperateDevice(
                DeviceReason::InvalidTargetEvent,
            ))
        }
    }

    pub fn normal(self) -> PciResult<Normal> {
        if let TargetEvent::Normal(normal) = self {
            Ok(normal)
        } else {
            Err(PciError::FailedOperateDevice(
                DeviceReason::InvalidTargetEvent,
            ))
        }
    }
}
