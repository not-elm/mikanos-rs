use core::cell::OnceCell;

use simple_fat::bpb::BpbFat32;
use simple_fat::dir::data::file::RegularFile;
use simple_fat::error::{FatDeviceError, FatResult};
use simple_fat::{Fat, FatDeviceAccessible};
use simple_fat::dir::data::DataEntries;

use crate::serial_println;

static FS: FileSystem = FileSystem::uninit();

pub fn init(fat_volume: *mut u8) {
    FS.init(fat_volume);
    let root_dir =
        FS.0.get()
            .unwrap()
            .root_dir()
            .unwrap();

    use simple_fat::dir::entry::short::ShortDirEntryReadable;

    for r in root_dir {
        serial_println!("{:?}", r.name_buff().unwrap());
    }
}


pub fn open_file(file_name: &str) -> FatResult<RegularFile<BpbFat32<FatDevice>>> {
    FS.0.get()
        .unwrap()
        .open_file(file_name)
}


pub fn root_dir() -> FatResult<DataEntries<BpbFat32<FatDevice>>> {
    FS.0.get()
        .unwrap()
        .root_dir()
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
