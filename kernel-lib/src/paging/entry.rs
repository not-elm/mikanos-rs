use alloc::vec;
use core::mem;

use modular_bitfield::bitfield;
use modular_bitfield::prelude::B3;
use modular_bitfield::specifiers::{B12, B40};

#[derive(Debug)]
#[repr(transparent)]
pub struct PageMapEntryPtr(u64);


impl PageMapEntryPtr {
    #[inline]
    pub const fn from_addr(addr: u64) -> PageMapEntryPtr {
        Self(addr)
    }


    #[inline]
    pub fn new() -> Self {
        let buff = vec![0; 64 * 512];
        let addr = buff.as_ptr() as u64;
        mem::forget(buff);

        Self::from_addr(addr)
    }


    #[inline]
    pub fn child(&mut self) -> Self {
        if self.read().present() {
            Self::from_addr(self.read().addr())
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
        self.set_addr(child.addr())
    }


    pub fn add(&self, index: usize) -> PageMapEntryPtr {
        Self::from_addr(self.0 + (index as u64) * 64)
    }


    pub fn update<F: FnOnce(&mut PageMapEntry)>(&mut self, f: F) {
        let mut entry = self.read();
        f(&mut entry);
        unsafe { core::ptr::write(self.0 as *mut PageMapEntry, entry) }
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
        unsafe { core::ptr::read(self.0 as *const PageMapEntry) }
    }
}


#[bitfield]
#[repr(u64)]
#[derive(Debug)]
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
    #[skip]
    __: B3,
    pub addr: B40,
    #[skip]
    __: B12,
}