#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use uefi::{prelude::*, table::boot::MemoryDescriptor};
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
    let entry_point = kernel::process::load_kernel(
        &mut root_dir,
        &"kernel.elf",
        &mut BootAllocator::new(&mut system_table),
    )
        .unwrap();

    let mut v = exit_boot_service(&system_table);

    if let Ok(_) = system_table.exit_boot_services(handle, v.as_mut_slice()) {
        core::mem::forget(v);
        entry_point.execute();
    }

    assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}

fn exit_boot_service(
    system_table: &SystemTable<Boot>,
) -> Vec<u8> {
    let memory_map_size = system_table.boot_services().memory_map_size().map_size;
    let descriptor_size = core::mem::size_of::<MemoryDescriptor>();
    vec![0u8; memory_map_size + descriptor_size * 12]
}
