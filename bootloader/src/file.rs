use alloc::vec::Vec;

use uefi::{CStr16, Handle};
use uefi::prelude::{Boot, SystemTable};
use uefi::proto::media::file::{Directory, File, FileAttribute, FileHandle, FileMode};

/// SimpleFileSystemについてはUEFI.mdを参照
#[allow(dead_code)]
pub(crate) fn open_root_dir(
    image_handle: Handle,
    system_table: &SystemTable<Boot>,
) -> uefi::Result<Directory> {
    return system_table
        .boot_services()
        .get_image_file_system(image_handle) // EfiImageHandleを元にSFSPを取得します。
        .map(|mut sfs| sfs.open_volume())?; // SFSPを用いてルートディレクトリを開きます。
}


#[allow(dead_code)]
pub(crate) fn open_file(
    dir: &mut Directory,
    file_name: &str,
    open_mode: FileMode,
) -> crate::error::Result<FileHandle> {
    // CStr16はすべての文字を16bitで表します
    let mut buff = Vec::<u16>::new();
    // 配列長の+1はバッファに十分なサイズを持たせる必要があるためです。
    buff.resize(file_name.chars().count() + 1, 0);
    let file_name_c_str = CStr16::from_str_with_buf(file_name, buff.as_mut_slice()).unwrap();
    let file_handle = dir.open(file_name_c_str, open_mode, FileAttribute::empty())?;
    Ok(file_handle)
}