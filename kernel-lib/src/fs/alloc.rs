use alloc::vec;
use core::mem::ManuallyDrop;

use common_lib::error::CommonResult;
use common_lib::loader;

use crate::paging::linear_address::LinearAddress;
use crate::paging::setup_page_maps;

pub struct FsAllocator;


impl loader::alloc::Allocatable for FsAllocator {
    #[inline]
    fn copy_mem(&mut self, dest: *mut u8, src: *const u8, size: usize) {
        unsafe {
            dest.copy_from(src, size);
        }
    }


    #[inline]
    fn set_mem(&mut self, buff: *mut u8, size: usize, value: u8) {
        unsafe {
            buff.write_bytes(value, size);
        }
    }


    fn allocate_pool(&self, size: usize) -> *mut u8 {
        let mut buff = ManuallyDrop::new(vec![0u8; size]);
        buff.as_mut_ptr()
    }


    #[inline]
    fn free_pool(&self, addr: *mut u8) {
        unsafe {
            addr.drop_in_place();
        }
    }


    #[inline]
    fn allocate_pages(&mut self, addr: u64, count: usize) -> CommonResult {
        setup_page_maps(
            LinearAddress::new(addr),
            count,
        );
        Ok(())
    }
}
