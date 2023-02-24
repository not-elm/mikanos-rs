use crate::elf::{Elf64Addr, Elf64Off};

/// ELFファイルのプログラムヘッダ
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Phdr {
    pub p_type: PType,
    //TODO
    pub p_flags: u32,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PType {
    /// この配列要素は指定されていません。
    /// Phdrのその他のメンバの値は未定義になります。
    PTNull = 0,

    /// この配列要素はp_fileszとp_memszで記述されるロード可能セグメントを指定します。
    /// syp_memszがファイルサイズより大きい場合は、余ったバイトは0にしなくてはいけません。
    PtLoad = 1,

    /// この配列要素は動的リンクを指定します。
    PtDynamic = 2,

    /// この配列要素はインタプリタとして起動されるパス名(NULL終端文字列)の
    /// 位置とサイズを指定します。
    PtInterp = 3,

    PtNote = 4,

    PtShlib = 5,
    PtPhdr = 6,
    PtLosunw = 0x6ffffffa,
    PtSunWbss = 0x6ffffffb,
    PtSunWdTrace = 0x6ffffffc,
    PtSunWCap = 0x6ffffffd,
    PtHiSun = 0x6fffffff,
    PtLoproc = 0x70000000,
    PtHiproc = 0x7fffffff,
}


