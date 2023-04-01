use xhci::ring::trb::event::TransferEvent;
use xhci::ring::trb::transfer::{DataStage, Normal, StatusStage};

use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug)]
pub enum EventTrb {
    TransferEvent {
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    },
    PortStatusChangeEvent(xhci::ring::trb::event::PortStatusChange),
    CommandCompletionEvent(xhci::ring::trb::event::CommandCompletion),
    NotSupport {
        trb_type: u8,
    },
}

#[derive(Debug)]
pub enum TargetEvent {
    Normal(Normal),
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
            3 => Some(TargetEvent::DataStage(
                DataStage::try_from(raw_data_data.into_u32_array()).ok()?,
            )),
            4 => Some(TargetEvent::StatusStage(
                StatusStage::try_from(raw_data_data.into_u32_array()).ok()?,
            )),
            _ => None,
        }
    }
}

impl EventTrb {
    pub unsafe fn new(trb: TrbRawData, cycle_bit: bool) -> Option<Self> {
        if read_cycle_bit(trb.raw()) != cycle_bit {
            return None;
        }

        let raw_data_buff: [u32; 4] = trb.into();

        let event_trb = match trb.template().trb_type() {
            32 => EventTrb::TransferEvent {
                transfer_event: TransferEvent::try_from(raw_data_buff).ok()?,
                target_event: read_target_trb(TransferEvent::try_from(raw_data_buff).ok()?)?,
            },
            33 => EventTrb::CommandCompletionEvent(
                xhci::ring::trb::event::CommandCompletion::try_from(raw_data_buff).ok()?,
            ),
            34 => EventTrb::PortStatusChangeEvent(
                xhci::ring::trb::event::PortStatusChange::try_from(raw_data_buff).ok()?,
            ),
            _ => EventTrb::NotSupport {
                trb_type: trb.template().trb_type(),
            },
        };

        Some(event_trb)
    }
}

fn read_target_trb(transfer: TransferEvent) -> Option<TargetEvent> {
    let raw_data = unsafe { *(transfer.trb_pointer() as *const u128) };
    let template = TrbRawData::new_unchecked(raw_data).template();

    match template.trb_type() {
        1 => Some(TargetEvent::Normal(
            xhci::ring::trb::transfer::Normal::try_from(
                TrbRawData::new_unchecked(raw_data).into_u32_array(),
            )
            .ok()?,
        )),
        3 => Some(TargetEvent::DataStage(
            DataStage::try_from(TrbRawData::new_unchecked(raw_data).into_u32_array()).ok()?,
        )),
        4 => Some(TargetEvent::StatusStage(
            StatusStage::try_from(TrbRawData::new_unchecked(raw_data).into_u32_array()).ok()?,
        )),
        _ => None,
    }
}

fn read_cycle_bit(trb: u128) -> bool {
    let cycle_bit = (trb >> 96) & 0b1;
    cycle_bit == 1
}
