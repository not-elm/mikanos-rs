use uefi::Handle;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::{Boot, SystemTable};
use uefi::table::boot::ScopedProtocol;

/// SimpleFileSystemについてはUEFI.mdを参照
pub(crate) fn open_root_dir(image_handle: Handle, system_table: &SystemTable<Boot>) -> uefi::Result<ScopedProtocol<SimpleFileSystem>>{
    return system_table
        .boot_services()
        .get_image_file_system(image_handle)
}