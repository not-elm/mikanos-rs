use alloc::format;

use uefi::prelude::{Boot, SystemTable};
use uefi::proto::media::file::{File, RegularFile};
use uefi::table::boot::MemoryDescriptor;

use crate::error::from_sfs_write_result;
use crate::{error, kib};

#[allow(dead_code)]
pub(crate) fn save_memory_map(
    mut file: RegularFile,
    system_table: &mut SystemTable<Boot>,
) -> error::Result<()> {
    let header = b"Index,Type,PhysicalStart,NumberOfPages,Attribute\n";
    file.write(header).unwrap();

    // 余裕を持たせるために16KiB
    const MEMORY_MAP_BUFF_SIZE: usize = kib!(16);
    let mut buff = [0u8; MEMORY_MAP_BUFF_SIZE];
    let (_, iter) = system_table
        .boot_services()
        .memory_map(&mut buff)
        .unwrap();

    unsafe {
        for (i, memory_descriptor) in iter.enumerate() {
            write_memory_descriptor_info(i, &mut file, memory_descriptor)?;
        }
    }
    file.flush()
        .map_err(|_| error::Error::Void)
}


unsafe fn write_memory_descriptor_info(
    i: usize,
    file: &mut RegularFile,
    memory_descriptor: &MemoryDescriptor,
) -> error::Result<()> {
    let mut index = format!("{} |", i);
    let mut memory_type = format!("{:?} | ", memory_descriptor.ty);
    let mut phys_start = format!("0x{:X} | ", memory_descriptor.phys_start);
    let mut page_count = format!("{} | ", memory_descriptor.page_count);
    let mut attribute = format!("{:?}\n", memory_descriptor.att);
    from_sfs_write_result(file.write(index.as_bytes_mut()))?;
    from_sfs_write_result(file.write(memory_type.as_bytes_mut()))?;
    from_sfs_write_result(file.write(phys_start.as_bytes_mut()))?;
    from_sfs_write_result(file.write(page_count.as_bytes_mut()))?;
    from_sfs_write_result(file.write(attribute.as_bytes_mut()))?;
    Ok(())
}
