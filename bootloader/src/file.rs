use uefi::fs::FileSystem;
use uefi::prelude::{Boot, SystemTable};
use uefi::Handle;

/// SimpleFileSystemについてはUEFI.mdを参照
#[allow(dead_code)]
pub(crate) fn open_file_system(
    image_handle: Handle,
    system_table: &SystemTable<Boot>,
) -> uefi::Result<FileSystem<'_>> {
    system_table
        .boot_services()
        .get_image_file_system(image_handle)
}
