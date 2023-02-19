#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]


use core::fmt::Write;

use uefi::prelude::*;

use crate::assembly::hlt_forever;

mod assembly;


#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Uefi-rs側でfmt::Writeトレートを実装しているため、writelnマクロが使えます。
    writeln!(system_table.stdout(), "Hello, Mikan Rust World!").unwrap();


    hlt_forever();
    #[allow(unreachable_code)]
    Status::SUCCESS
}

