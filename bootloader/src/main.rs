#![no_main]
#![no_std]


extern crate alloc;

use uefi::prelude::*;
use uefi::proto::media::file::FileMode;
use uefi_services::println;

use crate::memory_map::{open_file, open_root_dir, save_memory_map};

mod assembly;
mod memory_map;
mod unit;
mod error;


#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello, Mikan Rust World!");

    let mut root_dir = open_root_dir(handle, &system_table).unwrap();
    let mem_map_file = open_file(&mut root_dir, "mem_map", FileMode::CreateReadWrite).unwrap();
    save_memory_map(mem_map_file.into_regular_file().unwrap(), &mut system_table).unwrap();

    assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}

