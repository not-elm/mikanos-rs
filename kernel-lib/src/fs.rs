use core::arch::asm;
use core::cell::OnceCell;

use simple_fat::{Fat, FatDeviceAccessible};
use simple_fat::bpb::BpbFat32;
use simple_fat::dir::data::DataEntries;
use simple_fat::dir::data::file::RegularFile;
use simple_fat::error::FatDeviceError;

use common_lib::loader::elf::ElfLoader;
use common_lib::loader::ExecuteFileLoadable;

use crate::error::KernelResult;
use crate::fs::alloc::FsAllocator;
use crate::tss::STACK;

mod alloc;

static FS: FileSystem = FileSystem::uninit();

pub fn init(fat_volume: *mut u8) {
    FS.init(fat_volume);
}


pub fn open_file(file_name: &str) -> KernelResult<RegularFile<BpbFat32<FatDevice>>> {
    Ok(FS
        .0
        .get()
        .unwrap()
        .open_file(file_name)?)
}


pub fn root_dir() -> KernelResult<DataEntries<BpbFat32<FatDevice>>> {
    Ok(FS
        .0
        .get()
        .unwrap()
        .root_dir()?)
}


pub fn execute_elf_from_name(file_name: &str) -> KernelResult {
    let file = open_file(file_name)?;
    execute_elf(file)
}

// "CallApp:  ; void CallApp(int argc, char** argv, uint16_t cs, uint16_t ss, uint64_t rip, uint64_t rsp);
pub fn execute_elf(file: RegularFile<BpbFat32<FatDevice>>) -> KernelResult {
    let mut buff = file.read_boxed()?;

    let entry_point_addr = ElfLoader::new().load(&mut buff, &mut FsAllocator)?;

    let entry_point_ptr = *entry_point_addr as *const ();
    let entry_point: extern "sysv64" fn() -> () = unsafe { core::mem::transmute(entry_point_ptr) };
    unsafe {
        asm!(
        "push rbp",
        "mov rbp, rsp",
        "push {ss:r} //SS",
        "push {rsp:r} //RSP",
        "push {cs:r} //CS",
        "push {rip:r} //RIP",
        "retfq",
        cs = in(reg) 4 << 3 | 3,
        ss = in(reg) 3 << 3 | 3,
        rip = in(reg) *entry_point_addr,
        rsp = in(reg) STACK.as_ptr() as u64 + 4096  - 8
        )
    }
    // entry_point();
    Ok(())
}


#[derive(Debug)]
struct FileSystem(OnceCell<Fat<FatDevice>>);


impl FileSystem {
    pub const fn uninit() -> Self {
        Self(OnceCell::new())
    }


    pub fn init(&self, fat_volume: *mut u8) {
        self.0
            .set(Fat::new(FatDevice::new(fat_volume)))
            .unwrap();
    }
}


unsafe impl Sync for FileSystem {}


#[derive(Clone, Debug)]
pub struct FatDevice {
    fat_volume: *mut u8,
}


impl FatDevice {
    #[inline]
    pub const fn new(fat_volume: *mut u8) -> Self {
        Self { fat_volume }
    }
}


impl FatDeviceAccessible for FatDevice {
    fn read(&self, buff: &mut [u8], offset: usize, bytes: usize) -> Result<(), FatDeviceError> {
        unsafe {
            let src = core::slice::from_raw_parts(self.fat_volume.add(offset), bytes);
            buff.copy_from_slice(src);
        }

        Ok(())
    }


    fn write(&mut self, _buff: &[u8], _offset: usize) -> Result<(), FatDeviceError> {
        todo!()
    }
}


impl Drop for FatDevice {
    fn drop(&mut self) {
        unsafe {
            self.fat_volume
                .drop_in_place();
        }
    }
}
