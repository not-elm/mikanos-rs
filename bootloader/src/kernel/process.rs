use alloc::vec;
use alloc::vec::Vec;

use uefi::prelude::{Boot, SystemTable};
use uefi::proto::media::file::{Directory, File, FileInfo, FileMode, RegularFile};
use uefi::table::boot::MemoryDescriptor;

use bootloader_lib::error::LibResult;
use bootloader_lib::kernel::entry_point::EntryPoint;
use bootloader_lib::kernel::loaders::elf_loader::ElfLoader;
use bootloader_lib::kernel::loaders::{Allocatable, KernelLoadable};

use crate::file::open_file;
use crate::gop::{obtain_frame_buffer_config, open_gop};

pub fn load_kernel(
    root_dir: &mut Directory,
    kernel_file_path: &str,
    allocator: &mut impl Allocatable,
) -> LibResult<EntryPoint> {
    let mut kernel_file = open_file(root_dir, kernel_file_path, FileMode::Read)
        .map(|file_handle| file_handle.into_regular_file())
        .expect("should open kernel.libs")
        .unwrap();

    let kernel_file_size = get_kernel_file_size(&mut kernel_file) as usize;

    let mut kernel_vec = read_kernel_buff(&mut kernel_file, kernel_file_size);

    let result = ElfLoader::new().load(kernel_vec.as_mut_slice(), allocator);

    kernel_file.close();
    result
}

pub fn execute_kernel(entry_point: EntryPoint, system_table: SystemTable<Boot>) -> Result<(), ()> {
    let memory_map_vec = new_memory_map_vec(&system_table);

    let frame_buffer_config = obtain_frame_buffer_config(&mut open_gop(&system_table).unwrap());

    let (_, memory_map) = system_table.exit_boot_services();
    entry_point.execute(&frame_buffer_config, &memory_map.entries());
    core::mem::forget(memory_map_vec);
    Ok(())
}

fn get_kernel_file_size(kernel_file: &mut RegularFile) -> u64 {
    // カーネルファイルの大きさを知るため、ファイル情報を読み取る
    const FILE_INFO_SIZE: usize = 4000;

    let mut buff = vec![0u8; FILE_INFO_SIZE];
    let info = kernel_file
        .get_info::<FileInfo>(buff.as_mut_slice())
        .expect("should obtain kernel libs info");

    info.file_size()
}

fn read_kernel_buff(kernel_file: &mut RegularFile, kernel_file_size: usize) -> Vec<u8> {
    let mut v = vec![0; kernel_file_size];
    kernel_file
        .read(v.as_mut_slice())
        .unwrap();
    v
}

fn new_memory_map_vec(system_table: &SystemTable<Boot>) -> Vec<u8> {
    let memory_map_size = system_table
        .boot_services()
        .memory_map_size()
        .map_size;
    let descriptor_size = core::mem::size_of::<MemoryDescriptor>();
    vec![0u8; memory_map_size + descriptor_size * 12]
}
