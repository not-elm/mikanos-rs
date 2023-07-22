use alloc::boxed::Box;
use alloc::vec;
use core::cell::OnceCell;

use crate::serial_println;
use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment as TaskStateSegmentInner;
use x86_64::VirtAddr;

pub static TSS: TaskStateSegment = TaskStateSegment::new();

pub struct TaskStateSegment(OnceCell<TaskStateSegmentInner>);

lazy_static! {
    pub static ref STACK: Box<[u64]> = { vec![0; 8 * 4096].into_boxed_slice() };
}
impl TaskStateSegment {
    pub const fn new() -> Self {
        Self(OnceCell::new())
    }


    pub fn get(&self) -> &TaskStateSegmentInner {
        self.0.get_or_init(|| {
            let mut tss = TaskStateSegmentInner::new();

            tss.privilege_stack_table[0] =
                VirtAddr::new(STACK.as_ptr() as u64 + STACK.len() as u64);
            serial_println!("TSS {:?}", STACK.as_ptr());
            tss
        })
    }
}


unsafe impl Sync for TaskStateSegment {}
