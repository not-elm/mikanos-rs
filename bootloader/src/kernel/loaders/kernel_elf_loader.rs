use alloc::vec::Vec;

use libs::elf::elf_header::ehdr_ptr::EhdrPtr;
use libs::elf::phdr::{PType, Phdr};
use libs::elf::phdr_iter::{self, PhdrIter};
use uefi::data_types::PhysicalAddress;
use uefi::prelude::{Boot, SystemTable};
use uefi::proto::media::file::{File, FileInfo, RegularFile};
use uefi::table::boot::{AllocateType, MemoryType};
use uefi_services::println;

use crate::kernel::loaders::KernelLoadable;

pub struct KernelElfLoader {}

impl KernelLoadable for KernelElfLoader {
    fn load(
        &mut self,
        kernel_file: &mut RegularFile,
        system_table: &mut SystemTable<Boot>,
    ) -> crate::error::Result<u64> {
        let kernel_file_size = get_kernel_page_size(kernel_file) as usize;
        const KERNEL_BASE_ADDR: u64 = 0x100_000u64;
        let entry_point = KERNEL_BASE_ADDR as *mut u8;

        let buff: &mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(entry_point, kernel_file_size as usize) };
        system_table
            .boot_services()
            .allocate_pool(MemoryType::LOADER_DATA, kernel_file_size)
            .unwrap();

        read_kernel_buff(kernel_file, buff);

        let ehdr = EhdrPtr::from_file_buff(buff);
        let (load_segment_start_addr, load_segment_last_addr) =
            ehdr.phdr_iter().calc_load_address_range();

        allocate_pages(
            system_table,
            load_segment_start_addr,
            load_segment_last_addr,
        )
        .unwrap();
        let phdr_iter = ehdr.phdr_iter();
        copy_segments(&ehdr, phdr_iter, system_table);
        let entry_point_addr_ptr = (load_segment_start_addr + 24) as *const u64;
        Ok(unsafe{*entry_point_addr_ptr})
    }
}

fn get_kernel_page_size(kernel_file: &mut RegularFile) -> u64 {
    // カーネルファイルの大きさを知るため、ファイル情報を読み取る
    const FILE_INFO_SIZE: usize = 4000;

    let mut buff = Vec::<u8>::new();
    buff.resize(FILE_INFO_SIZE, 0);
    let info = kernel_file
        .get_info::<FileInfo>(buff.as_mut_slice())
        .expect("should obtain kernel libs info");

    info.file_size()
}

fn copy_segments(ehdr: &EhdrPtr, phdr_iter: PhdrIter, system_table: &mut SystemTable<Boot>) {
    let loads = phdr_iter.filter(|p| p.p_type == PType::PtLoad);

    for phdr in loads {
        println!("Phdr {:?}", phdr);
        copy_mem(ehdr, &phdr, system_table);
        set_mem(ehdr, &phdr, system_table);
    }
}

fn copy_mem(ehdr: &EhdrPtr, phdr: &Phdr, system_table: &mut SystemTable<Boot>) {
    let load_to_addr = phdr.p_vaddr as *mut u8;
    let src = ehdr.phdr_ptr_from(phdr.p_offset);
    unsafe {
        system_table
            .boot_services()
            .memmove(load_to_addr, src, phdr.p_filesz as usize);
    }
}
fn set_mem(ehdr: &EhdrPtr, phdr: &Phdr, system_table: &mut SystemTable<Boot>) {
    let remain_bytes = phdr.p_memsz - phdr.p_filesz;
    let buff = (phdr.p_vaddr + phdr.p_filesz) as *mut u8;
    unsafe {
        system_table
            .boot_services()
            .set_mem(buff, remain_bytes as usize, 0);
    }
}
fn calc_kernel_page_size(file_size: u64) -> usize {
    ((file_size + 0xfff) / 0x1000) as usize
}

fn allocate_pages(
    system_table: &mut SystemTable<Boot>,
    load_segment_start_addr: u64,
    load_segment_last_addr: u64,
) -> uefi::Result<PhysicalAddress> {
    let kernel_page_size = calc_kernel_page_size(load_segment_last_addr - load_segment_start_addr);
    println!("page_size: {:#08x}", kernel_page_size);
    system_table.boot_services().allocate_pages(
        AllocateType::Address(PhysicalAddress::from(load_segment_start_addr)),
        MemoryType::LOADER_DATA,
        kernel_page_size,
    )
}

fn read_kernel_buff(kernel_file: &mut RegularFile, buff: &mut [u8]) {
    kernel_file.read(buff).unwrap();
}
