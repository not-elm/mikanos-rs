use alloc::format;
use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};

use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B12, B3, B40};

use crate::allocator::FRAME_ITER;
use crate::serial_println;

#[repr(transparent)]
pub struct PageMapEntryPtr(u64);

impl Debug for PageMapEntryPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f
            .debug_struct("PageMapEntryPtr")
            .field("addr", &format!("0x{:X}", self.0))
            .field("entry", &self.read())
            .finish()
    }
}

#[repr(align(4096))]
struct A(Box<[u8]>);

impl PageMapEntryPtr {
    #[inline]
    pub const fn from_addr(addr: u64) -> PageMapEntryPtr {
        Self(addr)
    }


    #[inline]
    pub fn new() -> Self {
        let frame = unsafe { FRAME_ITER.0.get_mut().unwrap().next() };

        // let buff = ManuallyDrop::new(A(Vec:: [0u8; 4096].into_boxed_slice()));
        let addr = frame.unwrap().base_phys_addr().raw();

        Self::from_addr(addr)
    }


    pub fn child(&mut self) -> Self {
        if self.read().present() {
            serial_println!("ADDR = {:X} {:X}", self.read().addr(),  self.read().addr() << 12);
            Self::from_addr(self.read().addr() << 12)
        } else {
            let child = Self::new();
            self.set_child(&child);
            self.update(|et| {
                et.set_present(true);
            });
            child
        }
    }


    pub fn set_child(&mut self, child: &PageMapEntryPtr) {
        self.set_addr(child.0)
    }


    pub fn add(&self, index: usize) -> PageMapEntryPtr {
        Self::from_addr(self.0 + (index * core::mem::size_of::<u64>()) as u64)
    }


    pub fn update<F: FnOnce(&mut PageMapEntry)>(&mut self, f: F) {
        let mut entry = self.read();
        f(&mut entry);

        unsafe {
            (self.0 as *mut PageMapEntry).write_volatile(entry);
        }
    }


    fn set_addr(&mut self, addr: u64) {
        self.update(|entry| {
            entry.set_addr(addr);
        });
    }


    fn addr(&self) -> u64 {
        self.read().addr()
    }


    fn read(&self) -> PageMapEntry {
        unsafe { core::ptr::read(self.0 as *mut PageMapEntry) }
    }
}


#[bitfield]
#[repr(u64)]
pub struct PageMapEntry {
    pub present: bool,
    pub writable: bool,
    pub user: bool,
    pub pwrite_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub huga_page: bool,
    pub global: bool,

    _r: B3,
    pub addr: B40,
    #[skip]
    __: B12,
}


impl Debug for PageMapEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f
            .debug_struct("PageMapEntry")
            .field("present", &self.present())
            .field("addr", &format!("0x{:X}", self.addr() >> 12))
            .finish()
    }
}