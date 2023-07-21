use alloc::alloc::alloc;
use alloc::format;
use core::alloc::Layout;
use core::fmt::{Debug, Formatter};

use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B12, B3, B40};

#[repr(transparent)]
#[derive(Clone)]
pub struct PageMapEntryPtr(u64);

impl PageMapEntryPtr {
    pub fn free(self) {
        unsafe {
            let ptr = self.0 as *mut u64;
            if !ptr.is_null() {
                *ptr = 0;
                ptr.drop_in_place();
            }
        }
    }
}


impl Debug for PageMapEntryPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f
            .debug_struct("PageMapEntryPtr")
            .field("addr", &format!("0x{:X}", self.0))
            .field("entry", &self.read())
            .finish()
    }
}


impl PageMapEntryPtr {
    #[inline]
    pub const fn from_addr(addr: u64) -> PageMapEntryPtr {
        Self(addr)
    }


    #[inline]
    pub fn new() -> Self {
        unsafe {
            let buff = alloc(Layout::from_size_align(4096, 4096).unwrap());
            buff.write_bytes(0, 4096);
            Self::from_addr(buff as u64)
        }
    }


    #[inline]
    pub fn present(&self) -> bool {
        self.0 != 0 && unsafe { *(self.0 as *const bool) }
    }


    pub fn child(&self) -> Option<Self> {
        if self.present() {
            Some(Self::from_addr(self.read().addr() << 12))
        } else {
            None
        }
    }


    pub fn child_get_or_create(&mut self) -> Self {
        if self.read().present() {
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
        self.set_addr(child.0 >> 12)
    }


    pub fn entry(&self, index: usize) -> PageMapEntryPtr {
        Self::from_addr(self.0 + (index * core::mem::size_of::<PageMapEntry>()) as u64)
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


    fn read(&self) -> PageMapEntry {
        unsafe { core::ptr::read(self.0 as *mut PageMapEntry) }
    }
}


#[bitfield]
#[derive(Debug)]
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
    #[skip]
    __: B3,
    pub addr: B40,
    #[skip]
    __: B12,
}

//
// impl Debug for PageMapEntry {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         f
//             .debug_struct("PageMapEntry")
//             .field("present", &self.present())
//             .field("addr", &format!("0x{:X}", self.addr() >> 12))
//             .finish()
//     }
// }