use alloc::format;
use alloc::vec::Vec;

use uefi::{CStr16, Handle};
use uefi::proto::media::file::{Directory, File, FileAttribute, FileHandle, FileMode, RegularFile};
use uefi::table::{Boot, SystemTable};

use common::error::Error;
use common::kib;
use common::result::from_sfs_write_result;

/// SimpleFileSystemについてはUEFI.mdを参照
pub(crate) fn open_root_dir(image_handle: Handle, system_table: &SystemTable<Boot>) -> uefi::Result<Directory> {
    return system_table
        .boot_services()
        .get_image_file_system(image_handle)
        .map(|mut sfs| sfs.open_volume())?;
}

pub(crate) fn open_file(mut dir: Directory, file_name: &str) -> Result<FileHandle, uefi::Error> {
    // CStr16はすべての文字を16bitで表します
    let mut buff = Vec::<u16>::new();
    // 配列長の+1はバッファに十分なサイズを持たせる必要があるためです。
    buff.resize(file_name.chars().count() + 1, 0);
    let file_name_c_str = CStr16::from_str_with_buf(file_name, buff.as_mut_slice()).unwrap();
    dir.open(file_name_c_str, FileMode::CreateReadWrite, FileAttribute::empty())
}


pub(crate) fn save_memory_map(mut file: RegularFile, system_table: &mut SystemTable<Boot>) -> common::result::Result<()> {
    let header = b"Index, Type, PhysicalStart, NumberOfPages, Attribute\n";
    file.write(header).unwrap();

    // 余裕を持たせるために16KiB
    const MEMORY_MAP_BUFF_SIZE: usize = kib!(16);
    let mut buff = [0u8; MEMORY_MAP_BUFF_SIZE];
    let (_, iter) = system_table
        .boot_services()
        .memory_map(&mut buff)
        .unwrap();

    unsafe {
        for (i, memory_descriptor) in iter.into_iter().enumerate() {
            let mut index = format!("{} ", i);
            let mut memory_type = format!("{} ", memory_descriptor.ty.0);
            let mut phys_start = format!("{} ", memory_descriptor.phys_start);
            let mut page_count = format!("{} ", memory_descriptor.page_count);
            let mut attribute = format!("{:?}\n", memory_descriptor.att);
            from_sfs_write_result(file.write(index.as_bytes_mut()))?;
            from_sfs_write_result(file.write(memory_type.as_bytes_mut()))?;
            from_sfs_write_result(file.write(phys_start.as_bytes_mut()))?;
            from_sfs_write_result(file.write(page_count.as_bytes_mut()))?;
            from_sfs_write_result(file.write(attribute.as_bytes_mut()))?;
        }
    }
    file.flush()
        .map_err(|_| Error::Void)
}



