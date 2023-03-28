use core::fmt::{Debug, Formatter};

use kernel_lib::{println, serial_println};

use crate::error::{PciError, PciResult};
use crate::xhc::transfer::trb_buffer_from_address;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct TrbRawData(u128);

impl TrbRawData {
    pub fn new_unchecked(trb_raw_data: u128) -> Self {
        Self(trb_raw_data)
    }
    pub fn new(trb_raw_data: u128) -> PciResult<Self> {
        let last_offset = into_u32_array(trb_raw_data);
        println!("Receive  TRB = {:?}", last_offset);
        if last_offset[0] == 0 {
            Err(PciError::InvalidTrb(trb_raw_data))
        } else {
            Ok(Self(trb_raw_data))
        }
    }
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        trb_buffer_from_address(&mut self.0)
    }
    pub fn raw(&self) -> u128 {
        self.0
    }
}

impl Debug for TrbRawData {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let buff: [u32; 4] = into_u32_array(self.0);

        f.write_fmt(format_args!("Raw data = {:x} array = {:?}", self.0, buff))
    }
}

impl Into<[u32; 4]> for TrbRawData {
    fn into(self) -> [u32; 4] {
        into_u32_array(self.0)
    }
}

impl From<[u32; 4]> for TrbRawData {
    fn from(value: [u32; 4]) -> Self {
        TrbRawData::new_unchecked(into_u128(value))
    }
}

fn into_u128(raw_data: [u32; 4]) -> u128 {
    let mask = |raw_data: u32, shift: u128| (raw_data as u128) << (32 * shift);

    mask(raw_data[0], 3) | mask(raw_data[1], 2) | mask(raw_data[2], 1) | mask(raw_data[3], 0)
}

fn into_u32_array(raw_data: u128) -> [u32; 4] {
    let raw_data = (&raw_data as *const u128).cast::<u32>();
    unsafe {
        let get = |index: usize| raw_data.add(index).read_volatile();

        [get(0), get(1), get(2), get(3)]
    }
}

#[cfg(test)]
mod tests {
    use crate::xhc::transfer::trb_raw_data::TrbRawData;

    #[test]
    fn it_success_create_trb() {
        let raw_data = 0xFF_00_00_FFu128;
        assert!(TrbRawData::new(raw_data).is_ok());
    }

    #[test]
    fn it_success_into_u32_array() {
        let trb = TrbRawData::new_unchecked(0x3333_1111_0000_FFFFu128);
        let raw_data = (&trb.raw() as *const u128).cast::<u32>();
        let data_buff: [u32; 4] = trb.into();
        let expect_buff = unsafe { core::slice::from_raw_parts(raw_data, 4) };

        data_buff
            .iter()
            .zip(expect_buff)
            .for_each(|(data, trb_data)| {
                assert_eq!(*data, *trb_data);
            });
    }

    #[test]
    fn it_success_into_port_status_change_event_array() {
        let trb = TrbRawData::new_unchecked(0x8801010000000000000005000000u128);
        let trb_buff: [u32; 4] = trb.into();
        assert!(xhci::ring::trb::event::PortStatusChange::try_from(trb_buff).is_ok())
    }

    #[test]
    fn it_success_buffer_mut_port_status_change_event_array() {
        let mut trb = TrbRawData::new_unchecked(0x8801010000000000000005000000u128);
        let trb_buff: [u32; 4] = trb.into();
        let trb_buff_mut = trb.buffer_mut();
        let is_equal = trb_buff
            .iter()
            .zip(trb_buff_mut.iter())
            .all(|(x, y)| x == y);
        assert!(is_equal);
    }
}
