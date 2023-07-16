use uefi::prelude::Boot;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::table::SystemTable;

use bootloader_lib::error::{BootLoaderError, LibResult};
use bootloader_lib::kernel::loaders::Allocatable;

pub struct BootAllocator<'a>(&'a mut SystemTable<Boot>);

impl<'a> BootAllocator<'a> {
    pub fn new(system_table: &'a mut SystemTable<Boot>) -> Self {
        Self(system_table)
    }
}

impl Allocatable for BootAllocator<'_> {
    fn copy_mem(&self, dest: *mut u8, src: *const u8, size: usize) {
        unsafe {
            self.0
                .boot_services()
                .memmove(dest, src, size);
        }
    }

    fn set_mem(&mut self, buff: *mut u8, size: usize, value: u8) {
        unsafe {
            self.0
                .boot_services()
                .set_mem(buff, size, value);
        };
    }

    fn allocate_pool(&self, size: usize) -> *mut u8 {
        self.0
            .boot_services()
            .allocate_pool(MemoryType::LOADER_DATA, size)
            .unwrap()
    }

    fn free_pool(&self, addr: *mut u8) {
        self.0
            .boot_services()
            .free_pool(addr)
            .unwrap();
    }
    fn allocate_pages(&mut self, phys_addr: u64, count: usize) -> LibResult {
        self.0
            .boot_services()
            .allocate_pages(
                AllocateType::Address(phys_addr),
                MemoryType::LOADER_DATA,
                count,
            )
            .map_err(|_| BootLoaderError::FailedToAllocatePages(phys_addr))?;
        Ok(())
    }
}
