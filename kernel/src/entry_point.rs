/// カーネルのスタック領域を表します。
///
/// RSPレジスタがこのスタック領域を使用するために、
/// カーネルのエントリーポイントで
/// この領域の終了アドレスをRSPに書き込んでいます。
#[macro_export]
macro_rules! kernel_entry_point {
    () => {
        const KERNEL_STACK: KernelStack = KernelStack::new();

        #[repr(C, align(16))]
        struct KernelStack([u8; 1024 * 1024]);

        impl KernelStack {
            const fn new() -> Self {
                Self([0; 1024 * 1024])
            }
            pub fn end_addr(&self) -> usize {
                self.0.as_ptr().addr() + self.0.len()
            }
        }


        #[no_mangle]
        pub extern "sysv64" fn kernel_entry_point(
            frame_buffer_config: &common_lib::frame_buffer::FrameBufferConfig,
            memory_map: &uefi::table::boot::MemoryMapIter,
        ){
            let kernel_stack_end_addr = KERNEL_STACK.end_addr();

            unsafe {
                core::arch::asm!(
                    "mov rsp, {0}",
                    "call kernel_main",

                    in(reg) kernel_stack_end_addr,
                    in("rdi") frame_buffer_config,
                    in("esi") memory_map,
                    clobber_abi("sysv64")
                )
            }
        }
    };
}
