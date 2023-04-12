use core::mem::size_of;

use crate::paging::setup_identity_page_table;
use crate::segment::descriptor::SegmentDescriptor;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

mod descriptor;

static mut GDT: [u64; 8] = [0; 8];
//
// fn set_code_segment(
//     descriptor_type: u8,
//     privilege_level: usize,
//     base: u32,
//     limit: u32,
// ) -> SegmentDescriptor {
//     let mut descriptor = SegmentDescriptor::new();
//     descriptor.set_base_low((base & 0xFF_FF) as u16);
//     descriptor.set_base_middle(((base >> 16) & 0xFF) as u8);
//     descriptor.set_base_high(((base >> 24) & 0xFF) as u8);
//
//     descriptor.set_limit_low((limit & 0xFF_FF) as u16);
//     descriptor.set_limit_high(((limit >> 16) & 0xF) as u8);
//
//     descriptor.set_descriptor_type(descriptor_type);
//     descriptor.set_system_segment(true);
//     descriptor.set_descriptor_privilege_level(privilege_level);
//     descriptor.set_preset(true);
//     descriptor.set_long_mode(true);
//     descriptor.set_granularity(true);
//     descriptor
// }
//
// fn set_data_segment(
//     descriptor_type: u8,
//     privilege_level: usize,
//     base: u32,
//     limit: u32,
// ) -> SegmentDescriptor {
//     let mut descriptor = set_code_segment(descriptor_type, privilege_level,
// base, limit);     descriptor.set_long_mode(false);
//     descriptor.set_default_operation_size(true);
//     descriptor
// }
//
// extern "C" {
//     fn LoadGDT(limit: u16, offset: u64);
//     fn SetDSAll(value: u16);
//     fn SetCSSS(cs: u16, ss: u16);
// }
//
// pub unsafe fn setup_segments() {
//     GDT[0] = 0;
//     GDT[1] = set_code_segment(10, 0, 0, 0xF_FF_FF).into();
//     GDT[2] = set_data_segment(2, 0, 0, 0xF_FF_FF).into();
//     LoadGDT(
//         (GDT.len() * size_of::<u64>() - 1) as u16,
//         GDT.as_ptr() as u64,
//     );
//
//     const KERNEL_CS: u16 = 1 << 3;
//     const KERNEL_SS: u16 = 2 << 3;
//     SetDSAll(0);
//
//     SetCSSS(KERNEL_CS, KERNEL_SS);
//     setup_identity_page_table();
// }
