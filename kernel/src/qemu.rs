use pci::configuration_space::io::asm::io_out32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    io_out32(0xF4, exit_code as u32);

    common_lib::assembly::hlt_forever();
}
