use alloc::vec;
use alloc::vec::Vec;
use core::ffi::c_void;

use bootloader_lib::error::BootLoaderResult;
use uefi::fs::{FileSystem, Path};
use uefi::prelude::{Boot, SystemTable};
use uefi::table::boot::MemoryDescriptor;
use uefi::table::cfg::ACPI2_GUID;
use uefi::CString16;

use bootloader_lib::kernel::entry_point::EntryPoint;

use common_lib::error::CommonResult;
use common_lib::loader::elf::ElfLoader;
use common_lib::loader::{Allocatable, ExecuteFileLoadable};

use crate::gop::{obtain_frame_buffer_config, open_gop};

pub fn load_kernel(
    fs: &mut FileSystem,
    kernel_file_path: &str,
    allocator: &mut impl Allocatable,
) -> BootLoaderResult<EntryPoint> {
    let mut kernel_buff = fs.read(Path::new(&CString16::try_from("kernel.elf").unwrap()))?;
    let entry_point_addr = ElfLoader::new().load(kernel_buff.as_mut_slice(), allocator)?;

    Ok(EntryPoint::new(entry_point_addr))
}


pub fn execute_kernel(
    entry_point: EntryPoint,
    system_table: SystemTable<Boot>,
    fat_volume: *mut u8,
) -> Result<(), ()> {
    let memory_map_vec = new_memory_map_vec(&system_table);
    let frame_buffer_config = obtain_frame_buffer_config(&mut open_gop(&system_table).unwrap());
    let rsdp_ptr = find_rsdp_pointer(&system_table);
    let (_, memory_map) = system_table.exit_boot_services();

    entry_point.execute(
        &frame_buffer_config,
        &memory_map.entries(),
        &rsdp_ptr,
        fat_volume,
    );
    core::mem::forget(memory_map_vec);
    Ok(())
}


fn find_rsdp_pointer(system_table: &SystemTable<Boot>) -> Option<*const c_void> {
    system_table
        .config_table()
        .iter()
        .find(|config| config.guid == ACPI2_GUID)
        .map(|config| config.address)
}


fn new_memory_map_vec(system_table: &SystemTable<Boot>) -> Vec<u8> {
    let memory_map_size = system_table
        .boot_services()
        .memory_map_size()
        .map_size;
    let descriptor_size = core::mem::size_of::<MemoryDescriptor>();
    vec![0u8; memory_map_size + descriptor_size * 12]
}
