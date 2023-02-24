#![feature(pointer_byte_offsets)]

#![no_main]
#![no_std]


extern crate alloc;

use uefi::prelude::*;
use uefi_services::println;

use crate::file::open_root_dir;
use crate::kernel::loaders::kernel_elf_loader::KernelElfLoader;

mod assembly;
mod memory_map;
mod unit;
mod error;
mod kernel;
mod file;


#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello, Mikan Rust World!{}", libs::add(1, 2));

    let mut root_dir = open_root_dir(handle, &system_table).unwrap();
    kernel::start_kernel(&mut root_dir,
                         &"kernel.elf",
                         &mut KernelElfLoader {},
                         &mut system_table).unwrap();


    assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}


