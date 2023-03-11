use core::arch::global_asm;

use crate::assembly::config_address_param::ConfigAddrRegisterParam;

global_asm!(
    r#"
    asm_io_out32:
        mov dx, di 
        mov eax, esi
        out dx, eax
        ret
"#
);

global_asm!(
    r#"
    asm_io_in32:
        mov dx, di
        in eax, dx
        ret
"#
);

extern "C" {
    /// 1 .dx=di
    /// 2. eax=esi
    /// 3. IOポートのアドレスdxにeaxを出力
    /// 4. eaxの値がret(返り値になる)
    fn asm_io_out32(addr: u16, data: u32);

    /// 1 dx=di
    /// 2.dxのIOポートアドレスの値をeaxに読み込む
    /// 3.eaxの値がret(返り値になる)
    fn asm_io_in32(addr: u16) -> u32;
}

pub fn write_addr(param: ConfigAddrRegisterParam) {
    /// コンフィグアドレスレジスタ
    const CONFIG_ADDR: u16 = 0x0cF8;

    io_out32(CONFIG_ADDR, *param)
}

pub fn read_data() -> u32 {
    /// コンフィグデータレジスタ
    const CONFIG_DATA: u16 = 0x0cFC;

    io_in32(CONFIG_DATA)
}

fn io_out32(addr: u16, data: u32) {
    unsafe { asm_io_out32(addr, data) }
}

fn io_in32(addr: u16) -> u32 {
    unsafe { asm_io_in32(addr) }
}
