#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi_services::println;

use crate::file::open_root_dir;

mod assembly;
mod error;
mod file;
mod kernel;
mod memory_map;
mod unit;

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    use crate::kernel::boot_allocator::BootAllocator;
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello, Mikan Rust World!");

    let mut root_dir = open_root_dir(handle, &system_table).unwrap();
    kernel::process::execute_kernel(
        &mut root_dir,
        &"kernel.elf",
        &mut BootAllocator::new(&mut system_table),
    )
    .unwrap();

    assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}
