use crate::elf::phdr::Phdr;

pub struct PhdrIter {
    phdr_ptr: *mut Phdr,
    current_num: u16,
    e_phnum: u16,
}

impl PhdrIter {
    pub fn new(phdr_ptr: *mut Phdr, e_phnum: u16) -> Self {
        Self {
            phdr_ptr,
            current_num: 1,
            e_phnum,
        }
    }

    fn dref(&self) -> Phdr {
        unsafe { *(self.phdr_ptr) }
    }
}

impl Iterator for PhdrIter {
    type Item = Phdr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_num < self.e_phnum {
            let current = self.dref();
            unsafe {
                self.phdr_ptr = self.phdr_ptr.add(1);
                self.current_num += 1;
            }
            Some(current)
        } else if self.current_num == self.e_phnum {
            self.current_num += 1;
            let current = self.dref();
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::elf::elf_header::ehdr_ptr::EhdrPtr;
    use crate::elf::load_ehdr;
    use crate::elf::phdr::{PType, Phdr};

    #[test]
    fn it_cast_to_ehdr() {
        let mut phdr_iter = EhdrPtr::new(load_ehdr()).phdr_iter();
        let phdr = phdr_iter.next();
        assert!(phdr.is_some());
    }

    #[test]
    fn it_obtain_phdr_ptr() {
        let ehdr_ptr = EhdrPtr::new(load_ehdr());
        let phdr_page_num = ehdr_ptr.ph_num();
        let phdr_iter = ehdr_ptr.phdr_iter();

        let v: Vec<Phdr> = phdr_iter.collect();
        assert_eq!(phdr_page_num, 0x04);
        assert_eq!(v.len(), phdr_page_num as usize)
    }

    #[test]
    fn it_contains_two_load_segment() {
        let ehdr_ptr = EhdrPtr::new(load_ehdr());
        let phdr_iter = ehdr_ptr.phdr_iter();

        let v: Vec<Phdr> = phdr_iter.filter(|p| p.p_type == PType::PtLoad).collect();

        assert_eq!(v.len(), 2)
    }
}
