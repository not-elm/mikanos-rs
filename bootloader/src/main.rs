#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;


use core::fmt::Write;


use uefi::prelude::*;

use uefi::proto::media::file::File;
use uefi::proto::media::fs::SimpleFileSystem;

use uefi::table::boot::{ScopedProtocol};


fn get_memory_map(system_table: &mut SystemTable<Boot>){
    // let mut mem_map_buf = [0_u8; 4096 * 4];

    // let mem_map = {
    //     system_table
    //         .boot_services()
    //         .memory_map(&mut mem_map_buf)
    //         .unwrap()
    // };


}


 fn open_root_dir(handle: Handle, system_table: &BootServices) -> ScopedProtocol<SimpleFileSystem> {
    system_table
        .get_image_file_system(handle)
        .unwrap()
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();



    system_table.stdout().reset(false).unwrap();
    get_memory_map(&mut system_table);

    let  result =  {

        open_root_dir(_handle.clone(), system_table.boot_services()).open_volume().unwrap().handle().is_directory()
    };


    writeln!(system_table.stdout(), "debug {:?}", result).unwrap();
    Status::ABORTED
}