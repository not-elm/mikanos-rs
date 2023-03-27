/// カーネルのスタック領域を表します。
///
/// RSPレジスタがこのスタック領域を使用するために、カーネルのエントリーポイントで
/// この領域の終了アドレスをRSPに書き込んでいます。
pub const KERNEL_STACK: KernelStack = KernelStack::new();

#[repr(C, align(16))]
#[repr(C, align(16))]
pub struct KernelStack([u8; 1024 * 1024]);

impl KernelStack {
    const fn new() -> Self {
        Self([0; 1024 * 1024])
    }
    pub fn end_addr(&self) -> usize {
        self.0.as_ptr().addr() + self.0.len()
    }
}
