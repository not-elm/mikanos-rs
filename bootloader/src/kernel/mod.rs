use uefi::prelude::SystemTable;
use uefi::proto::media::file::{Directory, FileMode};
use uefi::table::Boot;
use uefi_services::println;

use crate::error;
use crate::file::open_file;
use crate::kernel::loaders::KernelLoadable;

pub mod loaders;


pub fn start_kernel(root_dir: &mut Directory,
                    kernel_file_path: &str,
                    kernel_loader: &mut dyn KernelLoadable,
                    system_table: &mut SystemTable<Boot>,
) -> error::Result {
    let mut kernel_file = open_file(root_dir, kernel_file_path, FileMode::Read)
        .map(|file_handle| file_handle.into_regular_file())
        .expect("should open kernel.libs")
        .unwrap();
    let entry_point = kernel_loader.load(&mut kernel_file, system_table).unwrap();


    println!("page_size: {:#08x}", unsafe { *entry_point });
    Ok(())
}


// fn start_kernel(virtual_address: *mut u8) {
//     let ptr = virtual_address as *const ();
//     let code: extern "C" fn() -> () = unsafe { core::mem::transmute(ptr) };
//     code();
// }
