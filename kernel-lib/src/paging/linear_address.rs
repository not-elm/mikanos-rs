#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LinearAddress(u64);


impl LinearAddress {
    pub const fn new(addr: u64) -> LinearAddress {
        Self(addr)
    }


    pub fn part(&self, page_map_level: usize) -> usize {
        let parts = self.read();

        match page_map_level {
            0 => parts.offset,
            1 => parts.page,
            2 => parts.dir,
            3 => parts.pdp,
            4 => parts.pml4,
            _ => 0
        }
    }

    fn read(&self) -> Parts {
        Parts {
            offset: (self.0 & 0b1111_1111_1111) as usize,
            page: ((self.0 >> 12) & 0b1_1111_1111) as usize,
            dir: ((self.0 >> 21) & 0b1_1111_1111) as usize,
            pdp: ((self.0 >> 30) & 0b1_1111_1111) as usize,
            pml4: ((self.0 >> 39) & 0b1_1111_1111) as usize,
        }
    }
}


struct Parts {
    offset: usize,
    page: usize,
    dir: usize,
    pdp: usize,
    pml4: usize,
}


