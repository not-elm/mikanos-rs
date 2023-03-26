use core::arch::global_asm;
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::{OffsetPageTable, PageTable, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::allo::BitmapMemoryManager;
use crate::error::KernelResult;
use crate::{println, serial_println};

const PAGE_SIZE_4K: usize = 4096;
pub const PAGE_SIZE_2M: usize = 512 * PAGE_SIZE_4K;
const PAGE_SIZE_1G: usize = 512 * PAGE_SIZE_2M;
const PAGE_DIRECTORY_COUNT: usize = 64;

static mut PML4_TABLE: Pml4Table = Pml4Table::new();
static mut PDR_TABLE: Pml4Table = Pml4Table::new();
static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new();
// static mut PAGE_TABLE: MyPageTable = MyPageTable::new();

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

pub unsafe fn setup_identity_page_table() {
    PML4_TABLE.0[0] = (PDR_TABLE.0.as_ptr() as u64 | 0x003);
    for pdr in 0..PAGE_DIRECTORY_COUNT {
        PDR_TABLE.0[pdr] = PAGE_DIRECTORY.0.as_ptr().add(pdr) as u64 | 0x003;
        for directory in 0..512 {
            PAGE_DIRECTORY.0[pdr][directory] =
                ((pdr * PAGE_SIZE_1G + directory * PAGE_SIZE_2M) as u64 | 0x083)
        }
    }

    serial_println!("PAGE_DIRECTORY 0 {}", PAGE_DIRECTORY.0[0][0]);
    // Cr3::write(
    //     PhysFrame::from_start_address(PhysAddr::new(PML4_TABLE.0.as_ptr().addr() as u64)).unwrap(),
    //     Cr3Flags::empty(),
    // );
    set_cr3(PML4_TABLE.0.as_ptr().addr() as u64);
}
global_asm!(
    r#"
   set_cr3:
        mov cr3, rdi
        ret
"#
);

extern "C" {
    fn set_cr3(value: u64);
}
pub fn make_identity_mapping(
    mapper: &mut OffsetPageTable,
    allocator: &mut BitmapMemoryManager,
    base_addr: u64,
    num_pages: usize,
) -> KernelResult {
    // use x86_64::structures::paging::PageTableFlags as Flags;
    // let base_page: Page<Size2MiB> = Page::from_start_address(VirtAddr::new(base_addr)).unwrap();
    // let base_frame = PhysFrame::from_start_address(PhysAddr::new(base_addr)).unwrap();
    // let flags = Flags::PRESENT | Flags::WRITABLE;
    // for i in 0..num_pages {
    //     let page = base_page + i as u64;
    //     let frame = base_frame + i as u64;
    //     let f = unsafe { mapper.map_to(page, frame, flags, &mut *allocator) };
    //     serial_println!("s: {:?}", f);
    //     if let Ok(f) = f {
    //         f.flush();
    //     }
    // }
    Ok(())
}

/// Initialize a new OffsetPageTable.
///
/// # Safety
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init_offset_page_table(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
    unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) }
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub(crate) unsafe fn active_level_4_table(
    physical_memory_offset: VirtAddr,
) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub fn table_addr() -> usize {
    unsafe { PML4_TABLE.0.as_ptr().addr() }
}
