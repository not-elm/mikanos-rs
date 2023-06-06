use crate::acpi::description_header;
use crate::acpi::description_header::DescriptionHeader;
use crate::error::KernelResult;

#[derive(Debug, Clone)]
pub struct Xsdt {
    index: u64,
    addr: u64,
    header: DescriptionHeader,
}


impl Xsdt {
    pub fn new(addr: u64) -> KernelResult<Self> {
        Ok(Self {
            index: 0,
            addr,
            header: DescriptionHeader::new_with_check(addr, "XSDT")?,
        })
    }


    pub fn fadt(mut self) -> Option<DescriptionHeader> {
        self.find(|header| header.valid_signature("FACP"))
    }
}


impl Iterator for Xsdt {
    type Item = DescriptionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.header.count() <= self.index {
            return None;
        }
        let entry_addr = self.addr + description_header::SIZE;
        let addr = entry_addr + (self.index * core::mem::size_of::<u64>() as u64);
        let header_addr = unsafe { *(addr as *const u64) };
        let header = DescriptionHeader::new(header_addr);

        self.index += 1;

        Some(header)
    }
}
