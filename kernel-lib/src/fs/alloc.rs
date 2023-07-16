use alloc::vec;

use common_lib::error::CommonResult;
use common_lib::loader;

pub struct FsAllocator;


impl loader::alloc::Allocatable for FsAllocator {
    #[inline]
    fn copy_mem(&self, dest: *mut u8, src: *const u8, size: usize) {
        unsafe { dest.copy_from(src, size) }
    }


    #[inline]
    fn set_mem(&mut self, buff: *mut u8, size: usize, value: u8) {
        unsafe {
            buff.write_bytes(value, size);
        }
    }


    fn allocate_pool(&self, size: usize) -> *mut u8 {
        let mut buff = vec![0; size];
        let ptr = buff.as_mut_ptr();

        core::mem::forget(buff);
        ptr
    }


    #[inline]
    fn free_pool(&self, addr: *mut u8) {
        unsafe {
            addr.drop_in_place();
        }
    }


    #[inline]
    fn allocate_pages(&mut self, _phys_addr: u64, _count: usize) -> CommonResult {
        Ok(())
    }
}
