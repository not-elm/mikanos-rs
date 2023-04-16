#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]
#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi::proto::media::file::FileMode;
use uefi_services::println;

use crate::file::{open_file, open_root_dir};
use crate::gop::{open_gop, write_all_pixels_with_same};
use crate::kernel::boot_allocator::BootAllocator;
use crate::memory_map::save_memory_map;

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

    let mut root_dir = open_root_dir(handle, &system_table).unwrap();
    save_memory_map(
        open_file(&mut root_dir, "memory_map", FileMode::CreateReadWrite)
            .unwrap()
            .into_regular_file()
            .unwrap(),
        &mut system_table,
    )
    .unwrap();

    let entry_point = kernel::process::load_kernel(
        &mut root_dir,
        &"kernel.elf",
        &mut BootAllocator::new(&mut system_table),
    )
    .unwrap();

    kernel::process::execute_kernel(entry_point, handle.clone(), system_table).unwrap();

    common_lib::assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}

#[allow(dead_code)]
fn fill_display_with_white(system_table: &SystemTable<Boot>) -> uefi::Result<()> {
    let mut gop = open_gop(&system_table)?;
    const WHITE_COLOR: u8 = 0xFF;
    unsafe {
        write_all_pixels_with_same(&mut gop, WHITE_COLOR);
    };
    Ok(())
}
