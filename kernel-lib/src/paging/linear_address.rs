use core::fmt::{Formatter, UpperHex};

use crate::paging::linear_address::parts::Parts;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LinearAddress(u64);


impl UpperHex for LinearAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}


impl LinearAddress {
    pub const fn new(addr: u64) -> LinearAddress {
        Self(addr)
    }


    pub fn part(&self, page_map_level: usize) -> usize {
        let parts = self.read();

        match page_map_level {
            0 => parts.offset() as usize,
            1 => parts.page() as usize,
            2 => parts.dir() as usize,
            3 => parts.pdp() as usize,
            4 => parts.pml4() as usize,
            _ => 0
        }
    }

    fn read(&self) -> Parts {
        Parts::from(self.0)
    }


    pub fn write(&mut self, level: usize, v: usize) {
        let mut parts = self.read();
        match level {
            0 => {
                parts.set_offset(v as u16);
            }
            1 => {
                parts.set_page(v as u16);
            }
            2 => {
                parts.set_dir(v as u16);
            }
            3 => {
                parts.set_pdp(v as u16);
            }
            4 => {
                parts.set_pml4(v as u16);
            }
            _ => {}
        }

        self.0 = parts.into();
    }
}


#[allow(unused)]
mod parts {
    use modular_bitfield::bitfield;
    use modular_bitfield::prelude::{B12, B16, B9};

    #[bitfield]
    #[repr(u64)]
    pub struct Parts {
        pub offset: B12,
        pub page: B9,
        pub dir: B9,
        pub pdp: B9,
        pub pml4: B9,
        #[skip]
        __: B16,
    }
}


