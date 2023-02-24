use crate::elf::elf_header::ehdr::Ehdr;
use crate::elf::phdr::Phdr;
use crate::elf::phdr_iter::PhdrIter;

#[repr(C)]
#[derive(Debug)]
pub struct EhdrPtr(*mut Ehdr);

impl EhdrPtr {
    pub fn new(ehdr_ptr: *mut Ehdr) -> Self {
        Self(ehdr_ptr)
    }
    pub fn from_file_buff(file_buff: &mut [u8]) -> Self {
        Self(Ehdr::from_file_buff(file_buff))
    }
    pub fn ph_num(&self) -> u16 {
        unsafe { *self.0 }.e_phnum
    }
    pub fn ph_offset(&self) -> u64 {
        unsafe { *self.0 }.e_phoff
    }

    pub fn phdr_iter(&self) -> PhdrIter{
        PhdrIter::new(self.phdr_ptr(), self.ph_num())
    }

    fn phdr_ptr(&self) -> *mut Phdr {
        let ph_offset = self.ph_offset();
        let ptr = self.0 as *mut u8;
        let phdr_start_addr = unsafe { ptr.byte_add(ph_offset as usize) };
        phdr_start_addr as *mut Phdr
    }
}


#[cfg(test)]
pub mod tests {
    use std::ptr::null_mut;

    use crate::elf::elf_header::ehdr_ptr::EhdrPtr;
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
}