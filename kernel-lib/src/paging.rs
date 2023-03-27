use core::arch::global_asm;

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

pub unsafe fn setup_identity_page_table() {
    PML4_TABLE.0[0] = PDR_TABLE.0.as_ptr() as u64 | 0x003;
    for pdr in 0..PAGE_DIRECTORY_COUNT {
        PDR_TABLE.0[pdr] = PAGE_DIRECTORY.0.as_ptr().add(pdr) as u64 | 0x003;
        for directory in 0..512 {
            PAGE_DIRECTORY.0[pdr][directory] =
                (pdr * PAGE_SIZE_1G + directory * PAGE_SIZE_2M) as u64 | 0x083
        }
    }

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
