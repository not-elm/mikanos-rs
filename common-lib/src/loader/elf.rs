use crate::elf::ehdr::elf_header_ptr::ElfHeaderPtr;
use crate::elf::phdr::program_header::{ProgramHeader, PType};
use crate::elf::phdr::program_header_table::ProgramHeaderTable;
use crate::error::CommonResult;
use crate::loader::{Allocatable, ExecuteFileLoadable};
use crate::loader::entry_point::EntryPointAddr;

#[derive(Debug, Clone)]
pub struct ElfLoader;


impl ElfLoader {
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}


impl Default for ElfLoader {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}


impl ExecuteFileLoadable for ElfLoader {
    fn load(
        &mut self,
        file_buff: &mut [u8],
        allocator: &mut impl Allocatable,
    ) -> CommonResult<EntryPointAddr> {
        let ehdr = ElfHeaderPtr::from_file_buff(file_buff);
        let (load_segment_start_addr, load_segment_last_addr) = ehdr
            .phdr_table()
            .calc_load_address_range();

        allocate_pages(allocator, load_segment_start_addr, load_segment_last_addr)?;
        let phdr_table = ehdr.phdr_table();

        copy_load_segments(&ehdr, phdr_table, allocator);

        let entry_point_addr_ptr = (load_segment_start_addr + 24) as *const u64;
        let entry_point_addr = unsafe { *entry_point_addr_ptr };
        Ok(EntryPointAddr::new(entry_point_addr))
    }
}


fn allocate_pages(
    system_table: &mut impl Allocatable,
    load_segment_start_addr: u64,
    load_segment_last_addr: u64,
) -> CommonResult {
    let kernel_page_size = calc_kernel_page_size(load_segment_last_addr - load_segment_start_addr);

    system_table.allocate_pages(load_segment_start_addr, kernel_page_size)
}


fn copy_load_segments(
    ehdr: &ElfHeaderPtr,
    phdr_table: ProgramHeaderTable,
    system_table: &mut impl Allocatable,
) {
    let phdr_iter_hold_loadable = phdr_table.filter(|p| p.p_type == PType::PtLoad);

    for phdr in phdr_iter_hold_loadable {
        copy_mem(ehdr, &phdr, system_table);
        set_zeros_if_over_file_size(&phdr, system_table);
    }
}


fn copy_mem(ehdr: &ElfHeaderPtr, phdr: &ProgramHeader, system_table: &mut impl Allocatable) {

    let load_destination_addr = phdr.p_vaddr as *mut u8;
    let loadable_segment = ehdr.segment_at(phdr.p_offset);
    system_table.copy_mem(
        load_destination_addr,
        loadable_segment,
        phdr.p_filesz as usize,
    );
}


/// セグメントのメモリ上のサイズがファイルサイズを超えている場合、
/// 超えた分だけ0を設定する必要があります。
fn set_zeros_if_over_file_size(phdr: &ProgramHeader, system_table: &mut impl Allocatable) {
    let remain_bytes = phdr.p_memsz - phdr.p_filesz;
    let buff = (phdr.p_vaddr + phdr.p_filesz) as *mut u8;
    system_table.set_mem(buff, remain_bytes as usize, 0);
}


fn calc_kernel_page_size(file_size: u64) -> usize {
    ((file_size + 0xfff) / 0x1000) as usize
}
