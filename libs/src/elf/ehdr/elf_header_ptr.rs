use crate::elf::ehdr::elf_header::ElfHeader;
use crate::elf::phdr::program_header::ProgramHeader;
use crate::elf::phdr::program_header_table::ProgramHeaderTable;

#[repr(C)]
#[derive(Debug)]
pub struct EhdrPtr(*mut ElfHeader);

impl EhdrPtr {
    pub fn new(ehdr_ptr: *mut ElfHeader) -> Self {
        Self(ehdr_ptr)
    }
    pub fn from_file_buff(file_buff: &mut [u8]) -> Self {
        Self(ElfHeader::from_file_buff(file_buff))
    }
    pub fn ph_num(&self) -> u16 {
        unsafe { *self.0 }.e_phnum
    }
    pub fn ph_offset(&self) -> u64 {
        unsafe { *self.0 }.e_phoff
    }
    pub fn phdr_ptr_from(&self, p_offset: u64) -> *const u8 {
        unsafe { self.0.byte_add(p_offset as usize) as *const u8 }
    }
    pub fn phdr_iter(&self) -> ProgramHeaderTable {
        ProgramHeaderTable::new(self.phdr_ptr(), self.ph_num())
    }

    fn phdr_ptr(&self) -> *mut ProgramHeader {
        let ph_offset = self.ph_offset();
        let ptr = self.0 as *mut u8;
        let phdr_start_addr = unsafe { ptr.byte_add(ph_offset as usize) };
        phdr_start_addr as *mut ProgramHeader
    }
}

#[cfg(test)]
pub mod tests {
    use core::ptr::null_mut;

    use crate::elf::ehdr::elf_header_ptr::EhdrPtr;
    use crate::elf::load_ehdr;

    #[test]
    fn it_obtain_program_header_offset() {
        let ptr = EhdrPtr(load_ehdr());
        assert_eq!(ptr.ph_offset(), 64);
    }

    #[test]
    fn it_obtain_program_headers_num() {
        let ptr = EhdrPtr(load_ehdr());
        assert_eq!(ptr.ph_num(), 4);
    }

    #[test]
    fn it_obtain_program_header_ptr() {
        let ptr = EhdrPtr::new(load_ehdr());
        let phdr = ptr.phdr_ptr();

        println!("{:?}", unsafe { *phdr });
        assert_ne!(phdr, null_mut());
    }

    #[test]
    fn it_obtain_program_header_ptr_from_p_offset() {
        let ptr = EhdrPtr::new(load_ehdr());
        let phdr = ptr.phdr_ptr();
        let p_offset = unsafe { (*phdr).p_offset };
        let expect = unsafe { *(phdr as *const u64) };
        let actual = unsafe { *(ptr.phdr_ptr_from(p_offset) as *const u64) };

        assert_eq!(actual, expect);
    }
}
