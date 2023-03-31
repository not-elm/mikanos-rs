use xhci::ring::trb::event::TransferEvent;
use xhci::ring::trb::transfer::StatusStage;

use kernel_lib::serial_println;

use crate::xhc::transfer::read_trb_type;
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
        trb_type: u128,
    },
}

#[derive(Debug)]
pub enum TargetEvent {
    StatusStage(xhci::ring::trb::transfer::StatusStage),
}

impl EventTrb {
    pub unsafe fn new(trb: TrbRawData, cycle_bit: bool) -> Option<Self> {
        if read_cycle_bit(trb.raw()) != cycle_bit {
            return None;
        }

        let raw_data_buff: [u32; 4] = trb.into();
        let event_trb = match read_trb_type(trb.raw()) {
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
                trb_type: read_trb_type(trb.raw()),
            },
        };

        Some(event_trb)
    }
}

fn read_target_trb(transfer: TransferEvent) -> Option<TargetEvent> {
    let raw_data = unsafe { *(transfer.trb_pointer() as *const u128) };
    serial_println!("Transfer Target Event Type = {}", read_trb_type(raw_data));
    match read_trb_type(raw_data) {
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

#[cfg(test)]
mod tests {
    use crate::xhc::transfer::event::event_trb::read_trb_type;

    #[test]
    fn it_trb_type_is_port_status_change() {
        let trb = 0x8801010000000000000005000000u128;
        assert_eq!(read_trb_type(trb), 34);
    }
}
