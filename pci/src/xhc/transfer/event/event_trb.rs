use crate::xhc::transfer::read_trb_type;
use kernel_lib::serial_println;

use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug)]
pub enum EventTrb {
    PortStatusChangeEvent(xhci::ring::trb::event::PortStatusChange),
    CommandCompletionEvent(xhci::ring::trb::event::CommandCompletion),
    NotSupport { trb_type: u128 },
}

impl EventTrb {
    pub unsafe fn new(trb: TrbRawData, cycle_bit: bool) -> Option<Self> {
        if read_cycle_bit(trb.raw()) != cycle_bit {
            return None;
        }
        serial_println!(
            "TRB Cycle Bit = {} Type={} Raw = {:?}",
            read_cycle_bit(trb.raw()),
            read_trb_type(trb.raw()),
            trb
        );
        let raw_data_buff: [u32; 4] = trb.into();
        let event_trb = match read_trb_type(trb.raw()) {
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

fn read_cycle_bit(trb: u128) -> bool {
    let cycle_bit = (trb >> 96) & 0b1;
    cycle_bit == 1
}

#[cfg(test)]
mod tests {
    use crate::xhc::transfer::event::event_trb::read_trb_type;
    use crate::xhc::transfer::read_trb_type;

    #[test]
    fn it_trb_type_is_port_status_change() {
        let trb = 0x8801010000000000000005000000u128;
        assert_eq!(read_trb_type(trb), 34);
    }
}
