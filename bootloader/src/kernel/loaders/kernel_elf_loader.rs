use alloc::vec::Vec;

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
    ) -> crate::error::Result<*mut u8> {
        let kernel_file_size = get_kernel_page_size(kernel_file);
        const KERNEL_BASE_ADDR: u64 = 0x100_000u64;
        let entry_point = KERNEL_BASE_ADDR as *mut u8;

        allocate_pages(system_table, KERNEL_BASE_ADDR, kernel_file_size).unwrap();

        let buff: &mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(entry_point, kernel_file_size as usize) };
        read_kernel_buff(kernel_file, buff);

        let ehdr = libs::elf::elf_header::ehdr::Ehdr::from_file_buff(buff);
        println!("elf = {:?}", unsafe { *ehdr });
        Ok(entry_point)
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

fn calc_kernel_page_size(file_size: u64) -> usize {
    ((file_size + 0xfff) / 0x1000) as usize
}

fn allocate_pages(
    system_table: &mut SystemTable<Boot>,
    kernel_base_addr: u64,
    kernel_file_size: u64,
) -> uefi::Result<PhysicalAddress> {
    let kernel_page_size = calc_kernel_page_size(kernel_file_size);
    println!("page_size: {:#08x}", kernel_page_size);
    system_table.boot_services().allocate_pages(
        AllocateType::Address(PhysicalAddress::from(kernel_base_addr)),
        MemoryType::LOADER_DATA,
        kernel_page_size,
    )
}

fn read_kernel_buff(kernel_file: &mut RegularFile, buff: &mut [u8]) {
    kernel_file.read(buff).unwrap();
}
