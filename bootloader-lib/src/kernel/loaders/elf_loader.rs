use crate::elf::ehdr::elf_header_ptr::ElfHeaderPtr;
use crate::elf::phdr::program_header::{PType, ProgramHeader};
use crate::elf::phdr::program_header_table::ProgramHeaderTable;
use crate::error::LibResult;
use crate::kernel::entry_point::EntryPoint;
use crate::kernel::loaders::{Allocatable, KernelLoadable};

pub struct ElfLoader {}

impl ElfLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ElfLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelLoadable for ElfLoader {
    fn load(
        &mut self,
        kernel_buff: &mut [u8],
        allocator: &mut impl Allocatable,
    ) -> LibResult<EntryPoint> {
        let ehdr = ElfHeaderPtr::from_file_buff(kernel_buff);
        let (load_segment_start_addr, load_segment_last_addr) =
            ehdr.phdr_iter().calc_load_address_range();

        allocate_pages(allocator, load_segment_start_addr, load_segment_last_addr)?;
        let phdr_iter = ehdr.phdr_iter();
        copy_segments(&ehdr, phdr_iter, allocator);
        let entry_point_addr_ptr = (load_segment_start_addr + 24) as *const u64;
        let entry_point_addr = unsafe { *entry_point_addr_ptr };
        Ok(EntryPoint::new(entry_point_addr))
    }
}

fn allocate_pages(
    system_table: &mut impl Allocatable,
    load_segment_start_addr: u64,
    load_segment_last_addr: u64,
) -> LibResult {
    let kernel_page_size = calc_kernel_page_size(load_segment_last_addr - load_segment_start_addr);

    system_table.allocate_pages(load_segment_start_addr, kernel_page_size)
}

fn copy_segments(
    ehdr: &ElfHeaderPtr,
    phdr_iter: ProgramHeaderTable,
    system_table: &mut impl Allocatable,
) {
    let loads = phdr_iter.filter(|p| p.p_type == PType::PtLoad);

    for phdr in loads {
        copy_mem(ehdr, &phdr, system_table);
        set_mem(&phdr, system_table);
    }
}

fn copy_mem(ehdr: &ElfHeaderPtr, phdr: &ProgramHeader, system_table: &mut impl Allocatable) {
    let load_to_addr = phdr.p_vaddr as *mut u8;
    let src = ehdr.phdr_ptr_from(phdr.p_offset);
    system_table.copy_mem(load_to_addr, src, phdr.p_filesz as usize);
}

fn set_mem(phdr: &ProgramHeader, system_table: &mut impl Allocatable) {
    let remain_bytes = phdr.p_memsz - phdr.p_filesz;
    let buff = (phdr.p_vaddr + phdr.p_filesz) as *mut u8;
    system_table.set_mem(buff, remain_bytes as usize, 0);
}

fn calc_kernel_page_size(file_size: u64) -> usize {
    ((file_size + 0xfff) / 0x1000) as usize
}