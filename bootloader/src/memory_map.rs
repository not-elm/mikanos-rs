use uefi::{CStr16, Error, Handle, Status};
use uefi::data_types::FromStrWithBufError;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileHandle, FileMode, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::{Boot, SystemTable};
use uefi::table::boot::ScopedProtocol;

/// SimpleFileSystemについてはUEFI.mdを参照
pub(crate) fn open_root_dir(image_handle: Handle, system_table: &SystemTable<Boot>) -> uefi::Result<Directory> {

    return system_table
        .boot_services()
        .get_image_file_system(image_handle)
        .map(|mut sfs| sfs.open_volume())?
}
pub(crate) fn open_file(mut dir: Directory) -> Result<FileHandle, uefi::Error> {
    const FILE_NAME: &str = "mem_map";
    // CStr16はすべての文字を16bitで表します。
    // ファイル名が7文字なのに対し、配列長が8なのは、十分な配列の長さを確保する必要があると
    // ドキュメントに記載されていたためです。
    let mut buff= [0u16; 8];
    let file_name_c_str = CStr16::from_str_with_buf(FILE_NAME, &mut buff).unwrap();
    dir.open(file_name_c_str, FileMode::CreateReadWrite, FileAttribute::empty())
}


pub(crate) fn save_memory_map(mut file: RegularFile) -> uefi::Result<(), usize> {
    let header = b"Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n";
    file.write(header)
}


