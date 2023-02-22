#![no_main]
#![no_std]


use uefi::prelude::*;
use uefi_services::println;

mod assembly;


#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello, Mikan Rust World!");

    assembly::hlt_forever();

    #[allow(unreachable_code)]
    Status::SUCCESS
}

