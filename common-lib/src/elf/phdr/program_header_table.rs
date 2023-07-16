use alloc::vec::Vec;
use core::cmp::Ordering;

use crate::elf::phdr::program_header::PType;
use crate::elf::phdr::program_header::ProgramHeader;

/// プログラムヘッダのヘッダテーブルはe_phnum個のphdrから構成される配列です。
/// e_phnumは、ElfHeader(Ehdr)内に宣言されています。
pub struct ProgramHeaderTable {
    phdr_ptr: *mut ProgramHeader,
    current_num: u16,
    e_phnum: u16,
}

impl ProgramHeaderTable {
    pub fn new(phdr_ptr: *mut ProgramHeader, e_phnum: u16) -> Self {
        Self {
            phdr_ptr,
            current_num: 1,
            e_phnum,
        }
    }

    /// ロード可能セグメント群の先頭アドレスと最終アドレス(最後のセグメントの終点)を取得します。
    /// 最初のセグメントのが示すアドレスがロード先の先頭アドレス、
    /// 最後のセグメントの先頭アドレス + メモリサイズがロード先の最終アドレスに対応します。
    pub fn calc_load_address_range(self) -> (u64, u64) {
        let mut v: Vec<ProgramHeader> = self.filter(|p| p.p_type == PType::PtLoad).collect();
        v.sort_by(|x1, x2|x1.p_vaddr.partial_cmp(&x2.p_vaddr).unwrap());

        let start_addr = v.first().unwrap().p_vaddr;
        let last_phdr = v.last().unwrap();
        let last_addr = last_phdr.p_vaddr + last_phdr.p_memsz;
        (start_addr, last_addr)
    }

    fn dref(&self) -> ProgramHeader {
        unsafe { *(self.phdr_ptr) }
    }
}

impl Iterator for ProgramHeaderTable {
    type Item = ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_num.cmp(&self.e_phnum) {
            Ordering::Less => {
                let current = self.dref();
                unsafe {
                    self.phdr_ptr = self.phdr_ptr.add(1);
                    self.current_num += 1;
                }
                Some(current)
            }
            Ordering::Equal => {
                self.current_num += 1;
                let current = self.dref();
                Some(current)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::elf::ehdr::elf_header_ptr::ElfHeaderPtr;
    use crate::elf::load_ehdr;
    use crate::elf::phdr::program_header::{PType, ProgramHeader};

    #[test]
    fn it_cast_to_ehdr() {
        let mut phdr_iter = ElfHeaderPtr::new(load_ehdr()).phdr_table();
        let phdr = phdr_iter.next();
        assert!(phdr.is_some());
    }

    #[test]
    fn it_obtain_phdr_ptr() {
        let ehdr_ptr = ElfHeaderPtr::new(load_ehdr());
        let phdr_page_num = ehdr_ptr.ph_num();
        let phdr_iter = ehdr_ptr.phdr_table();

        let v: Vec<ProgramHeader> = phdr_iter.collect();
        assert_eq!(phdr_page_num, 0x04);
        assert_eq!(v.len(), phdr_page_num as usize)
    }

    #[test]
    fn it_contains_two_load_segment() {
        let ehdr_ptr = ElfHeaderPtr::new(load_ehdr());
        let phdr_iter = ehdr_ptr.phdr_table();

        let v: Vec<ProgramHeader> = phdr_iter.filter(|p| p.p_type == PType::PtLoad).collect();

        assert_eq!(v.len(), 2)
    }

    #[test]
    fn it_success_calc_start_and_last_load_addresses() {
        let ehdr_ptr = ElfHeaderPtr::new(load_ehdr());
        let phdr_iter = ehdr_ptr.phdr_table();

        let (start, last) = phdr_iter.calc_load_address_range();
        assert_eq!(start, 0x100000u64);
        assert_eq!(last, 0x102000);
    }
}
