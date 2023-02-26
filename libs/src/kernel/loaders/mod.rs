use crate::error::LibResult;
use crate::kernel::entry_point::EntryPoint;

pub mod elf_loader;

pub trait Allocatable {
    fn copy_mem(&self, dest: *mut u8, src: *const u8, size: usize);
    fn set_mem(&mut self, buff: *mut u8, size: usize, value: u8);
    fn allocate_pool(&self, size: usize) -> *mut u8;
    fn free_pool(&self, addr: *mut u8);
    fn allocate_pages(&mut self, phys_addr: u64, count: usize) -> LibResult;
}

pub trait KernelLoadable {
    fn load(
        &mut self,
        kernel_buff: &mut [u8],
        allocator: &mut impl Allocatable,
    ) -> LibResult<EntryPoint>;
}
