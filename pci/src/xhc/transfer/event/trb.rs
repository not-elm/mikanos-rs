use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug)]
pub enum EventTrb {
    PortStatusChangeEvent(xhci::ring::trb::event::PortStatusChange),
    NotSupport { trb_type: u128 },
}

impl EventTrb {
    pub unsafe fn new(trb: TrbRawData) -> Option<Self> {
        let raw_data_buff: [u32; 4] = trb.into();
        let event_trb = match read_trb_type(trb.raw()) {
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

fn read_trb_type(trb: u128) -> u128 {
    trb >> 106
}

#[cfg(test)]
mod tests {
    use crate::xhc::transfer::event::trb::read_trb_type;

    #[test]
    fn it_trb_type_is_port_status_change() {
        let trb = 0x8801010000000000000005000000u128;
        assert_eq!(read_trb_type(trb), 34);
    }
}
