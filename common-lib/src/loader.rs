use crate::error::CommonResult;

use crate::loader::entry_point::EntryPointAddr;

pub mod alloc;
pub mod elf;
pub mod entry_point;

pub use crate::loader::alloc::Allocatable;


pub trait ExecuteFileLoadable {
    fn load(
        &mut self,
        file_buff: &mut [u8],
        allocator: &mut impl Allocatable,
    ) -> CommonResult<EntryPointAddr>;
}
