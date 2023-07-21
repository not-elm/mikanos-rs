use crate::control_registers::{read_cr3, set_cr3};
use crate::paging::entry::PageMapEntryPtr;
use crate::paging::linear_address::LinearAddress;

mod entry;
pub mod linear_address;

const PAGE_SIZE_4K: usize = 4096;
pub const PAGE_SIZE_2M: usize = 512 * PAGE_SIZE_4K;
const PAGE_SIZE_1G: usize = 512 * PAGE_SIZE_2M;
const PAGE_DIRECTORY_COUNT: usize = 64;

static mut PML4_TABLE: Pml4Table = Pml4Table::new();
static mut PDR_TABLE: Pml4Table = Pml4Table::new();
static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new();

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct Pml4Table([u64; 512]);

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct PageDirectory([[u64; 512]; PAGE_DIRECTORY_COUNT]);

impl PageDirectory {
    const fn new() -> Self {
        Self([[0u64; 512]; PAGE_DIRECTORY_COUNT])
    }
}

impl Pml4Table {
    const fn new() -> Self {
        Self([0; 512])
    }
}


pub fn setup_identity_page_table() {
    unsafe {
        PML4_TABLE.0[0] = PDR_TABLE.0.as_ptr() as u64 | 0x003;
        for pdr in 0..PAGE_DIRECTORY_COUNT {
            PDR_TABLE.0[pdr] = PAGE_DIRECTORY
                .0
                .as_ptr()
                .add(pdr) as u64
                | 0x003;
            for directory in 0..512 {
                PAGE_DIRECTORY.0[pdr][directory] =
                    (pdr * PAGE_SIZE_1G + directory * PAGE_SIZE_2M) as u64 | 0x083
            }
        }

        let cr3 = PML4_TABLE.0.as_ptr().addr() as u64;
        set_cr3(cr3);
        assert_eq!(read_cr3(), cr3);
    }
}


pub fn setup_page_maps(
    addr: LinearAddress,
    pages: usize,
) {
    let pml4_table = PageMapEntryPtr::from_addr(read_cr3());
    setup_page_map(pml4_table, 4, addr, pages);
}


fn setup_page_map(
    entry: PageMapEntryPtr,
    page_map_level: usize,
    mut addr: LinearAddress,
    pages: usize,
) -> usize {
    let mut pages = pages;

    while pages > 0 {
        let entry_idx = addr.part(page_map_level);
        let mut entry = entry.entry(entry_idx);
        let child = entry.child_get_or_create();

        entry.update(|et| {
            et.set_writable(true);
            et.set_present(true);
        });

        if page_map_level != 1 {
            pages = setup_page_map(child, page_map_level - 1, addr, pages);
        } else {
            pages -= 1;
        }

        if entry_idx == 511 {
            break;
        }

        addr.write(page_map_level, entry_idx + 1);
        for l in 0..page_map_level {
            addr.write(l, 0);
        }
    }

    pages
}


pub fn clean_page_maps(addr: LinearAddress) {
    let pml4_table = PageMapEntryPtr::from_addr(read_cr3());
    let pml4 = pml4_table.entry(addr.part(4));
    clean_page_map(pml4.clone(), 3);

    pml4.free();
}


pub fn clean_page_map(entries: PageMapEntryPtr, page_level: usize) {
    for i in 0..512 {
        let entry = entries.entry(i);
        if !entry.present() {
            continue;
        }

        if page_level > 1 {
            if let Some(child) = entry.child() {
                clean_page_map(child, page_level - 1);
            }
        }
    }
    entries.free();
}