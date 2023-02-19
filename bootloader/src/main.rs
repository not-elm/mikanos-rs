#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]


extern crate alloc;

use alloc::vec::Vec;
use core::fmt::Write;

use uefi::data_types::PhysicalAddress;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileInfo, FileMode, FileProtocolInfo};
use uefi::table::boot::{AllocateType, MemoryType};

use common::assembly::hlt_forever;

use crate::memory_map::{open_file, open_root_dir, save_memory_map};

mod memory_map;


#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Uefi-rs側でfmt::Writeトレートを実装しているため、writelnマクロが使えます。
    writeln!(system_table.stdout(), "Hello, Mikan Rust World!").unwrap();

    let mut root_dir = open_root_dir(_handle.clone(), &system_table).unwrap();
    let file_handle = open_file(&mut root_dir, "mem_map", FileMode::CreateReadWrite);

    let valid_save_memory_map_file = save_memory_map(file_handle.unwrap().into_regular_file().unwrap(), &mut system_table).is_ok();
    writeln!(system_table.stdout(), "Valid save Memory map={}", valid_save_memory_map_file).unwrap();

    let mut kernel_file = open_file(&mut root_dir, "\\kernel.elf", FileMode::Read)
        .map(|file_handle| file_handle.into_regular_file())
        .expect("should open kernel.elf")
        .unwrap();

    // カーネルファイルの大きさを知るため、ファイル情報を読み取る
    const FILE_INFO_SIZE: usize = 4000;
    let mut buff = Vec::<u8>::new();
    buff.resize(FILE_INFO_SIZE, 0);
    let info = kernel_file
        .get_info::<FileInfo>(buff.as_mut_slice())
        .expect("should obtain kernel elf info");


    // カーネルファイル全体を読み込むためのメモリを確保
    system_table
        .boot_services()
        .allocate_pages(AllocateType::Address(PhysicalAddress::from(0x100000u64)), MemoryType::LOADER_DATA, ((info.file_size() + 0xfff) / 0x1000) as usize)
        .expect("should be allocated pages");
    let kernel_file_size = info.file_size() as usize;
    let kernel_buff = system_table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, kernel_file_size)
        .unwrap();
    let entry_point = (kernel_buff as u64 + 24u64) as *const u64;
    let kernel_buff = unsafe { core::slice::from_raw_parts_mut(kernel_buff, kernel_file_size) };


    let red_size = kernel_file.read(kernel_buff).expect("should be read kernel data");
    kernel_file.close();
    writeln!(system_table.stdout(), "red kernel file bytes={}", red_size).expect("should be red read kernel data");
    let buf = unsafe { core::slice::from_raw_parts((p + 24) as *mut u8, 8) };
    let kernel_main_addr = LittleEndian::read_u64(&buf);

    // カーネルの起動
    let size = system_table.boot_services().memory_map_size().map_size;
    let mut buff = Vec::new();
    buff.resize(size + 0x100, 0);
    system_table.exit_boot_services(_handle, buff.as_mut_slice()).expect("should be exited boot service");

    start_kernel(entry_point);

    hlt_forever();
    Status::SUCCESS
}

fn start_kernel(virtual_address: *const u64) {
    let ptr = virtual_address as *const ();
    let code: extern "sysv64" fn() -> () = unsafe { core::mem::transmute(ptr) };
    (code)();
}