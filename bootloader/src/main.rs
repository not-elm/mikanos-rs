#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]



extern crate alloc;


use core::fmt::Write;


use uefi::prelude::*;
use common::assembly::hlt;
use common::kib_from_mb;


#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Uefi-rs側でfmt::Writeトレートを実装しているため、writelnマクロが使えます。
    writeln!(system_table.stdout(), "Hello, Mikan Rust World!").unwrap();
    writeln!(system_table.stdout(), "1KIB={}", kib_from_mb!(1)).unwrap();


    loop {
        unsafe {hlt();};
    }
    Status::SUCCESS
}