#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]
#![no_main]
#![no_std]

extern crate alloc;

use uefi::fs::Path;
use uefi::prelude::*;
use uefi::CString16;
use uefi_services::println;

use crate::file::open_file_system;
use crate::gop::{open_gop, write_all_pixels_with_same};
use crate::kernel::boot_allocator::BootAllocator;

mod error;
mod file;
mod gop;
mod kernel;
mod memory_map;
mod unit;

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello, Mikan Rust World!");

    let mut disk_buff = system_table
        .boot_services()
        .get_image_file_system(handle)
        .unwrap()
        .read(Path::new(&CString16::try_from("fat_disk").unwrap()))
        .unwrap();

    let entry_point = kernel::process::load_kernel(
        &mut open_file_system(handle, unsafe { &system_table.unsafe_clone() }).unwrap(),
        "kernel.elf",
        &mut BootAllocator::new(&mut system_table),
    )
    .unwrap();


    kernel::process::execute_kernel(entry_point, system_table, disk_buff.as_mut_ptr()).unwrap();

    common_lib::assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}


#[allow(dead_code)]
fn fill_display_with_white(system_table: &SystemTable<Boot>) -> uefi::Result<()> {
    let mut gop = open_gop(system_table)?;
    const WHITE_COLOR: u8 = 0xFF;
    unsafe {
        write_all_pixels_with_same(&mut gop, WHITE_COLOR);
    };
    Ok(())
}
