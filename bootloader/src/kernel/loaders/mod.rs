use uefi::proto::media::file::RegularFile;
use uefi::table::{Boot, SystemTable};

pub mod kernel_elf_loader;

pub trait KernelLoadable {
    fn load(
        &mut self,
        kernel_file: &mut RegularFile,
        system_table: &mut SystemTable<Boot>,
    ) -> crate::error::Result<u64>;
}
