extern crate alloc;

use alloc::alloc::Layout;
use core::alloc::GlobalAlloc;
use core::cell::UnsafeCell;
use core::ptr;

pub(crate) struct BumpPointerAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}

impl BumpPointerAlloc {
    pub(crate) const fn new(head: usize, end: usize) -> Self {
        Self {
            head: UnsafeCell::new(head),
            end,
        }
    }
}

impl BumpPointerAlloc {
    pub fn init(&mut self, head: usize, end: usize) {
        *self.head.get_mut() = head;
        self.end = end;
    }
}
unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = self.head.get();
        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.end {
            // ヌルポインタはメモリ不足の状態を知らせます
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // このアロケータはメモリを解放しません
    }
}
