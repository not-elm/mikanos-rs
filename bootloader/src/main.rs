#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]


extern crate alloc;

use core::fmt::Write;

use uefi::prelude::*;

use common::assembly::hlt_forever;

use crate::memory_map::{open_file, open_root_dir, save_memory_map};

mod memory_map;


#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Uefi-rs側でfmt::Writeトレートを実装しているため、writelnマクロが使えます。
    writeln!(system_table.stdout(), "Hello, Mikan Rust World!").unwrap();

    let root_dir = open_root_dir(_handle.clone(), &system_table).unwrap();
    let file_handle = open_file(root_dir, "mem_map");

    let valid_save_memory_map_file = save_memory_map(file_handle.unwrap().into_regular_file().unwrap(), &mut system_table).is_ok();
    writeln!(system_table.stdout(), "Valid save Memory map={}", valid_save_memory_map_file).unwrap();

    hlt_forever();
    Status::SUCCESS
}